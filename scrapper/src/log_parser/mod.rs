use common::Failure;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "log_parser/failed_tests.pest"]
pub struct FailedTestsParser;

impl FailedTestsParser {
    pub fn parse_failed_tests(input: &str) -> Vec<Failure> {
        let pairs = FailedTestsParser::parse(Rule::LOG, input).unwrap_or_else(|e| panic!("{}", e));

        pairs
            .flat_map(|pair| match pair.as_rule() {
                Rule::FAILURE => {
                    let mut pairs = pair.into_inner();
                    let step = pairs.next().unwrap().as_str().to_string();
                    let scenario = pairs.next().unwrap().as_str().to_string();

                    Some(Failure { scenario, step })
                }
                _ => None,
            })
            .collect()
    }
}
