use std::{
    fs::File,
    io::{BufWriter, Write},
};

use anyhow::Result;
use clap::Parser;
use common::{FailedJob, FailedRun};
use futures::future;
use github::{client::GithubClient, workflow::Job};
use tokio::task::JoinHandle;

mod github;
mod log_parser;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(long, short)]
    run_id: u64,

    #[clap(long, short = 't')]
    github_token: String,

    #[clap(long)]
    owner_repo: String,

    #[clap(long, short, default_value = "runs.json")]
    output: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let run_id = cli.run_id;

    let client = GithubClient::new(cli.github_token, cli.owner_repo);

    let failed_jobs = client.get_failed_workflow_jobs(cli.run_id).await?;

    let loged_failures = parse_logs(failed_jobs, &client).await;

    save_to_file(
        FailedRun {
            run_id,
            jobs: loged_failures,
        },
        cli.output,
    )
    .await?;

    Ok(())
}

async fn save_to_file(loged_failure: FailedRun, output: String) -> Result<()> {
    let file = File::open(&output)?;
    let prev_failures: Vec<FailedRun> = serde_json::from_reader(file)?;

    let mut failed_runs = vec![loged_failure];
    failed_runs.extend(prev_failures);

    let file = File::create(&output)?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &failed_runs)?;
    writer.flush()?;

    Ok(())
}

async fn parse_logs(jobs: Vec<Job>, client: &GithubClient) -> Vec<FailedJob> {
    let tasks = jobs
        .into_iter()
        .map(|job| {
            let client = client.clone();
            tokio::spawn(async move {
                let logs = client.download_job_logs(&job).await.unwrap();
                FailedJob {
                    job: job.name,
                    failures: log_parser::FailedTestsParser::parse_failed_tests(&logs),
                }
            })
        })
        .collect::<Vec<JoinHandle<FailedJob>>>();

    future::try_join_all(tasks).await.unwrap()
}
