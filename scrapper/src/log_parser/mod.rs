//! Module for parsing log files of failed jobs.
use common::{FailedScenario, Failure};
use pest::Parser;
use pest_derive::Parser;

/// Struct to parse failed tests from log files basef of [``pest``]grammar
#[derive(Parser)]
#[grammar = "log_parser/failed_tests.pest"]
pub(crate) struct FailedTestsParser;

impl FailedTestsParser {
    #[allow(clippy::unwrap_used)]
    /// Parses the failed tests from the log file. Also trys to build a link to
    /// failed feature using the `owner_repo`.
    pub(crate) fn parse_failed_tests(
        input: &str,
        owner_repo: &str,
    ) -> Vec<Failure> {
        let pairs =
            Self::parse(Rule::LOG, input).unwrap_or_else(|e| panic!("{}", e));

        pairs
            .filter_map(|pair| match pair.as_rule() {
                Rule::FAILURE => {
                    let mut failure_pairs = pair.into_inner();
                    let step =
                        failure_pairs.next().unwrap().as_str().to_owned();

                    let mut scenario =
                        failure_pairs.next().unwrap().into_inner();
                    let scenario_part =
                        scenario.next().unwrap().as_str().to_owned();
                    let mut path_part =
                        scenario.next().unwrap().as_str().to_owned();

                    let path = path_part
                        .split('\n')
                        .nth(0)
                        .unwrap()
                        .split("..")
                        .nth(1);
                    if let Some(path) = path {
                        path_part = format!(
                            "https://github.com/{}/blob/master{}",
                            owner_repo,
                            path.strip_suffix(":0").unwrap()
                        );
                    }

                    Some(Failure {
                        scenario: FailedScenario {
                            name: scenario_part,
                            link: path_part,
                        },
                        step,
                    })
                }
                Rule::EOI
                | Rule::TRASH
                | Rule::FAILED_TEST
                | Rule::FAILED_STEP
                | Rule::FAILED_STEP_MSG
                | Rule::SCENARIO_PART
                | Rule::PATH_PART
                | Rule::FAILED_SCENARIO_MSG
                | Rule::LOG => None,
            })
            .collect()
    }
}
