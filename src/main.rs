use std::{
    fs::File,
    io::{BufWriter, Write},
};

use anyhow::Result;
use clap::Parser;
use futures::future;
use github::{client::GithubClient, workflow::Job};
use log_parser::Failure;
use rayon::iter::{IntoParallelRefIterator, ParallelDrainRange, ParallelIterator};
use tokio::task::JoinHandle;

mod github;
mod log_parser;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(long, short)]
    workflow_id: u64,

    #[clap(long, short = 't')]
    github_token: String,

    #[clap(long, short)]
    owner: String,

    #[clap(long, short)]
    repo: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let client = GithubClient::new(cli.github_token, cli.owner, cli.repo);

    let failed_jobs = client.get_failed_workflow_jobs(cli.workflow_id).await?;

    let loged_failures = parse_logs(failed_jobs, &client).await;

    let file = File::create("a")?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &loged_failures)?;
    writer.flush()?;

    Ok(())
}

async fn parse_logs(jobs: Vec<Job>, client: &GithubClient) -> Vec<(Job, Vec<Failure>)> {
    let tasks = jobs
        .into_iter()
        .map(|job| {
            let client = client.clone();
            tokio::spawn(async move {
                let logs = client.download_job_logs(&job).await.unwrap();
                (
                    job,
                    log_parser::FailedTestsParser::parse_failed_tests(&logs),
                )
            })
        })
        .collect::<Vec<JoinHandle<(Job, Vec<Failure>)>>>();

    future::try_join_all(tasks).await.unwrap()
}
