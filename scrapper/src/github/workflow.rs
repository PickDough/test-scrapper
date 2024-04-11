use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct WorkflowRun {
    pub jobs: Vec<Job>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    pub id: u64,
    pub name: String,
    conclusion: Conclusion,
    steps: Vec<Step>,
}

impl Job {
    pub fn into_failed_job(mut self) -> Option<Self> {
        if self.conclusion.has_failed() {
            self.steps.retain(|step| step.conclusion.has_failed());

            Some(self)
        } else {
            None
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Step {
    name: String,
    conclusion: Conclusion,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Conclusion {
    Success,
    Failure,
    Skipped,
}

impl Conclusion {
    fn has_failed(&self) -> bool {
        match self {
            Conclusion::Success => false,
            Conclusion::Failure => true,
            Conclusion::Skipped => false,
        }
    }
}
