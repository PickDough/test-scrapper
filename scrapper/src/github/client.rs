//! Module to interact with the Github API
use std::mem;

use anyhow::Result;
use async_recursion::async_recursion;
use reqwest::{
    header::{ACCEPT, AUTHORIZATION, USER_AGENT},
    Method, Request,
};

use super::workflow::{Job, WorkflowRun};

/// A client to interact with the Github API.
#[derive(Debug, Clone)]
pub(crate) struct GithubClient {
    /// The HTTP client.
    client: reqwest::Client,
    /// The Github token.
    token: String,
    /// The owner and repository name in the format `owner/repo`.
    pub(crate) owner_repo: String,
}

impl GithubClient {
    /// Create a new Github client.
    pub(crate) fn new(token: String, owner_repo: String) -> Self {
        let client = reqwest::Client::new();

        Self {
            client,
            token,
            owner_repo,
        }
    }

    /// Get the failed jobs from a workflow run.
    pub(crate) async fn get_failed_workflow_jobs(
        &self,
        workflow_id: u64,
    ) -> Result<Vec<Job>> {
        let url = format!(
            "https://api.github.com/repos/{owner_repo}\
            /actions/runs/{workflow_id}/jobs",
            owner_repo = self.owner_repo
        );
        let req = self.build_request(&url)?;

        let mut failed_jobs = vec![];

        self.paginated_request(req, |text| {
            let response = serde_json::from_str::<WorkflowRun>(&text);
            if let Ok(res) = response {
                let failed = res
                    .jobs
                    .into_iter()
                    .filter_map(Job::into_failed_job)
                    .collect::<Vec<_>>();

                failed_jobs.extend(failed);
            }
        })
        .await?;

        Ok(failed_jobs)
    }

    /// Download the logs of a job.
    pub(crate) async fn download_job_logs(&self, job: &Job) -> Result<String> {
        let url = format!(
            "https://api.github.com/repos/{owner_repo}\
            /actions/jobs/{job_id}/logs",
            owner_repo = self.owner_repo,
            job_id = job.id
        );

        let req = self.build_request(&url)?;
        let response = self.client.execute(req).await?;

        Ok(response.text().await?)
    }

    /// Build a request with the necessary headers.
    fn build_request(&self, url: &str) -> Result<Request, anyhow::Error> {
        let mut req = Request::new(Method::GET, url.try_into()?);
        drop(
            req.headers_mut()
                .insert(ACCEPT, "application/vnd.github+json".try_into()?),
        );
        drop(
            req.headers_mut()
                .insert("X-GitHub-Api-Version", "2022-11-28".try_into()?),
        );
        drop(req.headers_mut().insert(
            AUTHORIZATION,
            format!("Bearer {}", self.token).try_into()?,
        ));
        drop(
            req.headers_mut()
                .insert(USER_AGENT, "test-scrapper 0.1.0".try_into()?),
        );
        Ok(req)
    }

    /// Make a paginated request. Making requests until there are no more pages.
    /// The `f` function is called with the response text.
    #[async_recursion(?Send)]
    async fn paginated_request<F>(
        &self,
        mut request: Request,
        mut f: F,
    ) -> Result<()>
    where
        F: FnMut(String),
    {
        if let Some(req) = request.try_clone() {
            let response = self.client.execute(req).await?;

            let next = response
                .headers()
                .get("Link")
                .and_then(|link| {
                    let link = link.to_str();
                    link.map_or(None, |s| {
                        let next =
                            s.split(',').find(|l| l.contains("rel=\"next\""));
                        next.map(|l| l.split(';').next().map(str::trim))
                    })
                })
                .and_then(|link| {
                    link.map(|l| {
                        l.trim_matches(|c| c == '<' || c == '>').to_owned()
                    })
                });

            f(response.text().await?);

            if let Some(next) = next {
                let _unused =
                    mem::replace(request.url_mut(), next.as_str().try_into()?);
                return self.paginated_request(request, f).await;
            }
        }
        Ok(())
    }
}
