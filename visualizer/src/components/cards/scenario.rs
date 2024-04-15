use yew::prelude::*;

#[function_component]
pub fn CardBodyScenario(props: &PropsScenario) -> Html {
    let classes =
        classes!("card", format!("text-bg-{}", props.style.clone()), "mb-3");
    html! {
        <div class={classes}>
            <div class="card-body">
                <span
                    class="card-text d-flex justify-content-center"
                    type="button"
                    data-bs-toggle="collapse"
                    data-bs-target={format!("#collapseExample{}", props.id)}
                    aria-expanded="false"
                    aria-controls={format!("collapseExample{}", props.id)}
                >
                    {props.scenario.clone()}
                    <div class="dropdown-toggle">
                    {" "}
                    </div>
                </span>
                <div class="collapse"
                    id={format!("collapseExample{}", props.id)}>
                    <div class="card card-body">
                        <ul class="list-group list-group-flush">
                            <li class="list-group-item">
                                <a href={props.link.clone()} target="_blank">
                                 {"Feature"}
                                </a>
                            </li>
                            <li class="list-group-item">
                                <pre>
                                    <code>
                                        {props.steps.clone()}
                                    </code>
                                </pre>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    }
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
    #[prop_or(AttrValue::from(""))]
    pub id: AttrValue,
}
