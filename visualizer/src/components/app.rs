//! App component. Groups and renders scenarios.
use crate::components::cards::{
    header::CardHeader, percentage::CardBodyPercentage,
    scenario::CardBodyScenario, test::CardBodyTest,
};
use std::collections::HashMap;

use common::FailedRun;
use gloo_net::http::Request;
use yew::prelude::*;

/// Hash Key for the failed run. Uniquely identifies by scenario and job.
#[derive(Hash, Eq, PartialEq, Debug)]
struct FailedRunKey {
    /// Scenario name.
    scenario: String,
    /// Job name.
    job: String,
}

/// Hash Value for the failed run. Contains steps, link and count of totatl
/// occurences compared to total amount of runs.
struct FailedRunValue {
    /// Steps of the scenario.
    steps: String,
    /// Link to the feature file.
    link: String,
    /// Count of total occurences.
    count: u16,
}

#[function_component]
pub fn App() -> Html {
    let failed_runs: UseStateHandle<Vec<FailedRun>> = use_state(Vec::new);
    {
        let failed_runs = failed_runs.clone();
        use_effect_with((), move |()| {
            wasm_bindgen_futures::spawn_local(async move {
                let res = Request::get("/test-scrapper/public/runs.json")
                    .send()
                    .await;

                if let Ok(fut_runs) = res {
                    let runs = fut_runs.json().await;
                    if let Ok(runs) = runs {
                        failed_runs.set(runs);
                    }
                }
            });
            || ()
        });
    }
    let mut failed_runs_map: HashMap<FailedRunKey, FailedRunValue> =
        HashMap::new();
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
        let _unused = failed_runs_map
            .entry(key)
            .and_modify(|v| v.count += 1)
            .or_insert(value);
    }

    #[allow(clippy::as_conversions, clippy::cast_precision_loss)]
    let total_count = failed_runs_map.len() as f64;
    let mut failed_runs_vec = failed_runs_map.iter().collect::<Vec<_>>();
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
                    failed_runs_vec
                        .into_iter()
                        .enumerate()
                        .map(|(id, (key, value))|
                    {
                    let percentage =
                        (f64::from(value.count) / total_count * 100f64)
                        .to_string();
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
                            <CardBodyTest>
                                {&key.job}
                            </CardBodyTest>
                        </div>
                        <div class="col-2">
                            <CardBodyPercentage style="danger"
                                percentage={percentage.clone()}>
                                {percentage.clone().push('%')}
                            </CardBodyPercentage>
                        </div>
                    </>
                    }
                }).collect::<Html>()
            }
            </div>
        </div>
    }
}
