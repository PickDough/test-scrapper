use std::mem;

use anyhow::Result;
use async_recursion::async_recursion;
use bytes::Bytes;
use reqwest::{
    header::{ACCEPT, AUTHORIZATION, USER_AGENT},
    Method, Request,
};

use super::workflow::{Job, WorkflowRun};

#[derive(Debug, Clone)]
pub struct GithubClient {
    client: reqwest::Client,

    token: String,
    owner: String,
    repo: String,
}

impl GithubClient {
    pub fn new(token: String, owner: String, repo: String) -> Self {
        let client = reqwest::Client::new();

        Self {
            client,
            token,
            owner,
            repo,
        }
    }

    pub async fn get_failed_workflow_jobs(&self, workflow_id: u64) -> Result<Vec<Job>> {
        let url = format!(
            "https://api.github.com/repos/{owner}/{repo}/actions/runs/{workflow_id}/jobs",
            owner = self.owner,
            repo = self.repo,
            workflow_id = workflow_id
        );
        let req = self.build_request(url)?;

        let mut failed_jobs = vec![];

        self.paginated_request(req, |text| {
            let response = serde_json::from_str::<WorkflowRun>(&text).unwrap();
            let failed = response
                .jobs
                .into_iter()
                .filter_map(Job::into_failed_job)
                .collect::<Vec<_>>();

            failed_jobs.extend(failed);
        })
        .await?;

        Ok(failed_jobs)
    }

    pub async fn download_job_logs(&self, job: &Job) -> Result<String> {
        let url = format!(
            "https://api.github.com/repos/{owner}/{repo}/actions/jobs/{job_id}/logs",
            owner = self.owner,
            repo = self.repo,
            job_id = job.id
        );

        let req = self.build_request(url)?;
        let res = self.client.execute(req).await?;

        Ok(res.text().await?)
    }

    fn build_request(&self, url: String) -> Result<Request, anyhow::Error> {
        let mut req = Request::new(Method::GET, url.as_str().try_into()?);
        req.headers_mut()
            .insert(ACCEPT, "application/vnd.github+json".try_into()?);
        req.headers_mut()
            .insert("X-GitHub-Api-Version", "2022-11-28".try_into()?);
        req.headers_mut()
            .insert(AUTHORIZATION, format!("Bearer {}", self.token).try_into()?);
        req.headers_mut()
            .insert(USER_AGENT, "test-scrapper 0.1.0".try_into()?);
        Ok(req)
    }

    #[async_recursion(?Send)]
    async fn paginated_request<F: FnMut(String) -> ()>(
        &self,
        mut request: Request,
        mut f: F,
    ) -> Result<()> {
        let response = self.client.execute(request.try_clone().unwrap()).await?;

        let next = response
            .headers()
            .get("Link")
            .and_then(|link| {
                let link = link.to_str().unwrap();
                let next = link.split(',').find(|link| link.contains("rel=\"next\""));
                next.map(|link| link.split(';').next().unwrap().trim())
            })
            .map(|link| link.trim_matches(|c| c == '<' || c == '>').to_owned());

        f(response.text().await?);

        if next.is_some() {
            let _ = mem::replace(request.url_mut(), next.unwrap().as_str().try_into()?);
            self.paginated_request(request, f).await
        } else {
            Ok(())
        }
    }
}
