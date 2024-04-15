use common::{FailedScenario, Failure};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "log_parser/failed_tests.pest"]
pub struct FailedTestsParser;

impl FailedTestsParser {
    pub fn parse_failed_tests(input: &str, owner_repo: &str) -> Vec<Failure> {
        let pairs = FailedTestsParser::parse(Rule::LOG, input)
            .unwrap_or_else(|e| panic!("{}", e));

        pairs
            .flat_map(|pair| match pair.as_rule() {
                Rule::FAILURE => {
                    let mut pairs = pair.into_inner();
                    let step = pairs.next().unwrap().as_str().to_string();

                    let mut scenario = pairs.next().unwrap().into_inner();
                    let scenario_part =
                        scenario.next().unwrap().as_str().to_string();
                    let mut path_part =
                        scenario.next().unwrap().as_str().to_string();

                    let path = path_part
                        .split("\n")
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
                _ => None,
            })
            .collect()
    }
}
