//! GitHub Workflow Run data structures.
use serde::{Deserialize, Serialize};

/// Workflow run data structure. Contains a list of jobs runned by workflow.
#[derive(Debug, Deserialize)]
pub(crate) struct WorkflowRun {
    /// The list of jobs.
    pub(crate) jobs: Vec<Job>,
}

/// Workflow job data structure. Contains a list of steps runned by job. And
/// their status.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Job {
    /// Id of the job.
    pub(crate) id: u64,
    /// Name of the job.
    pub(crate) name: String,
    /// Conclusion of the job.
    conclusion: Conclusion,
    /// Steps that job performed.
    steps: Vec<Step>,
}

impl Job {
    /// Looks for failed steps in job. If threre are any, returns Some(self).
    pub(crate) fn into_failed_job(mut self) -> Option<Self> {
        self.conclusion.has_failed().then(|| {
            self.steps.retain(|step| step.conclusion.has_failed());
            self
        })
    }
}

/// Workflow step data structure. Contains the name of the step and its status.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Step {
    /// Name of the step.
    name: String,
    /// Conclusion of the step.
    conclusion: Conclusion,
}

/// Conclusion of a job or step.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Conclusion {
    /// The job or step was successful.
    Success,
    /// The job or step failed.
    Failure,
    /// The job or step was skipped.
    Skipped,
}

impl Conclusion {
    /// Returns true if the job or step has [``Conclusion::Failure``] type.
    const fn has_failed(&self) -> bool {
        match self {
            Self::Failure => true,
            Self::Skipped | Self::Success => false,
        }
    }
}
