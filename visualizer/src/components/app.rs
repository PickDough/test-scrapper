use std::fs::File;

use crate::components::card_body::{CardBody, CardBodyScenario};

use super::card_header::CardHeader;
use common::FailedRun;
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    // let file = File::open("./runs.json").unwrap();
    // let prev_failures: Vec<FailedRun> = serde_json::from_reader(file).unwrap();

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
                <div class="col-7">
                    <CardBodyScenario
                        style="light"
                        scenario="Scenario Member disables audio before call"
                        link="https://github.com/instrumentisto/medea-jason/blob/master/e2e/tests/features/media_disable.feature"
                        steps="Then Alice's device video remote track from Bob is enabled # :0 took 30000ms  \n2024-03-22T18:09:43.0030721Z TimeoutException after 0:00:30.000000: Future not completed\n2024-03-22T18:09:43.0032631Z "
                        />
                </div>
                <div class="col-3">
                    <CardBody>
                        {"test (E2E, ios)"}
                    </CardBody>
                </div>
                <div class="col-2">
                    <CardBody style="danger">
                        {"10%"}
                    </CardBody>
                </div>
            </div>
            <div class="row">
                <div class="col-7">
                    <CardBody>
                        {"Scenario Member disables audio before call"}
                    </CardBody>
                </div>
                <div class="col-3">
                    <CardBody>
                        {"test (E2E, ios)"}
                    </CardBody>
                </div>
                <div class="col-2">
                    <CardBody style="danger">
                        {"10%"}
                    </CardBody>
                </div>
            </div>
        </div>
    }
}
