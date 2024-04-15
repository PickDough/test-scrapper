//! Scenario card component.
use yew::prelude::*;

#[function_component]
pub(crate) fn CardBodyScenario(props: &PropsScenario) -> Html {
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

/// Properties for the scenario card component.
#[derive(Properties, PartialEq)]
pub(crate) struct PropsScenario {
    /// The style of the card. Default is `light`. Look at Bootstrap colors for
    /// more information.
    #[prop_or(AttrValue::from("light"))]
    pub(crate) style: AttrValue,
    /// The steps of the scenario.
    #[prop_or(AttrValue::from(""))]
    pub(crate) steps: AttrValue,
    /// The link to the feature file.
    #[prop_or(AttrValue::from(""))]
    pub(crate) link: AttrValue,
    /// The scenario name.
    #[prop_or(AttrValue::from(""))]
    pub(crate) scenario: AttrValue,
    /// The unique id of the dropdown to toggle.
    #[prop_or(AttrValue::from(""))]
    pub(crate) id: AttrValue,
}
