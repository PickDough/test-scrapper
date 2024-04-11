use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FailedRun {
    pub run_id: u64,
    pub jobs: Vec<FailedJob>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FailedJob {
    pub job: String,
    pub failures: Vec<Failure>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Failure {
    pub scenario: String,
    pub step: String,
}
