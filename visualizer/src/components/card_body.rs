use yew::prelude::*;

#[function_component]
pub fn CardBody(props: &Props) -> Html {
    html! {
        <div class={classes!("card", format!("text-bg-{}", props.style.clone()), "mb-3")}>
            <div class="card-body">
                <p class="card-text">{ props.children.clone() }</p>
            </div>
        </div>
    }
}

#[function_component]
pub fn CardBodyScenario(props: &PropsScenario) -> Html {
    html! {
        <div class={classes!("card", format!("text-bg-{}", props.style.clone()), "mb-3")}>
            <div class="card-body">
                <span class="card-text d-flex justify-content-center" type="button" data-bs-toggle="collapse" data-bs-target="#collapseExample" aria-expanded="false" aria-controls="collapseExample">
                    {props.scenario.clone()}
                    <div class="dropdown-toggle">
                    {" "}
                    </div>
                </span>
                <div class="collapse" id="collapseExample">
                    <div class="card card-body">
                        <ul class="list-group list-group-flush">
                            <li class="list-group-item"><a href={props.link.clone()} target="_blank">{"Feature"}</a></li>
                            <li class="list-group-item"><pre><code>{props.steps.clone()}</code></pre></li>
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(AttrValue::from("light"))]
    pub style: AttrValue,
    pub children: Html,
}
#[derive(Properties, PartialEq)]
pub struct PropsScenario {
    #[prop_or(AttrValue::from("light"))]
    pub style: AttrValue,
    #[prop_or(AttrValue::from(""))]
    pub steps: AttrValue,
    #[prop_or(AttrValue::from(""))]
    pub link: AttrValue,
    #[prop_or(AttrValue::from(""))]
    pub scenario: AttrValue,
}
