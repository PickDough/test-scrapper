use std::{collections::HashMap, fs::File};

use crate::components::card_body::{CardBody, CardBodyScenario};

use super::card_header::CardHeader;
use common::FailedRun;
use gloo_net::http::Request;
use yew::prelude::*;

#[derive(Hash, Eq, PartialEq, Debug)]
struct FailedRunKey {
    scenario: String,
    job: String,
}
struct FailedRunValue {
    steps: String,
    link: String,
    count: u16,
}

#[function_component]
pub fn App() -> Html {
    // let file = File::open("./runs.json").unwrap();
    // let prev_failures: Vec<FailedRun> = serde_json::from_reader(file).unwrap();

    let failed_runs = use_state(|| vec![]);
    {
        let failed_runs = failed_runs.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_runs: Vec<FailedRun> = Request::get("/static/runs.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                failed_runs.set(fetched_runs);
            });
            || ()
        });
    }
    let mut failed_runs_map: HashMap<FailedRunKey, FailedRunValue> = HashMap::new();
    for (job, failed_scenario) in failed_runs
        .iter()
        .flat_map(|r| &r.jobs)
        .flat_map(|j| j.failures.iter().map(|f| (j.job.clone(), f)))
    {
        let key = FailedRunKey {
            scenario: failed_scenario.scenario.name.clone(),
            job: job.clone(),
        };
        let value = FailedRunValue {
            steps: failed_scenario.step.clone(),
            link: failed_scenario.scenario.link.clone(),
            count: 1,
        };
        failed_runs_map
            .entry(key)
            .and_modify(|v| v.count += 1)
            .or_insert(value);
    }

    let total_count = failed_runs_map.len() as f64;
    let mut failed_runs_vec = failed_runs_map
        .iter()
        .map(|(key, value)| (key, value))
        .collect::<Vec<_>>();
    failed_runs_vec.sort_by(|(_, a), (_, b)| b.count.cmp(&a.count));

    html! {
        <div class="container text-center p-3" style="width: 60%">
            <div class="row">
                <div class="col-7">
                    <CardHeader>
                        {"Scenario"}
                    </CardHeader>
                </div>
                <div class="col-3">
                    <CardHeader>
                        {"Test"}
                    </CardHeader>
                </div>
                <div class="col-2">
                    <CardHeader>
                        {"Statistics"}
                    </CardHeader>
                </div>
            </div>
            <div class="row">
                {
                    failed_runs_vec.into_iter().enumerate().map(|(id, (key, value))| {
                    html! {
                        <>
                        <div class="col-7">
                            <CardBodyScenario
                                style="light"
                                scenario={key.scenario.clone()}
                                link={value.link.clone()}
                                steps={value.steps.clone()}
                                id={id.to_string()}

                                />
                        </div>
                        <div class="col-3">
                            <CardBody>
                                {&key.job}
                            </CardBody>
                        </div>
                        <div class="col-2">
                            <CardBody style="danger">
                                {format!("{}%", value.count as f64 / total_count * 100f64)}
                            </CardBody>
                        </div>
                    </>
                    }
                }).collect::<Html>()
            }
            </div>
        </div>
    }
}
