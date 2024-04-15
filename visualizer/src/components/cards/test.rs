//! Test card component.
use yew::prelude::*;

#[function_component]
pub(crate) fn CardBodyTest(props: &PropsTest) -> Html {
    let classes =
        classes!("card", format!("text-bg-{}", props.style.clone()), "mb-3");
    html! {
        <div class={classes}>
            <div class="card-body">
                <p class="card-text">{ props.children.clone() }</p>
            </div>
        </div>
    }
}

/// Properties for the test card component.
#[derive(Properties, PartialEq)]
pub(crate) struct PropsTest {
    /// The style of the card. Default is `light`. Look at Bootstrap colors for
    /// more information.
    #[prop_or(AttrValue::from("light"))]
    pub(crate) style: AttrValue,
    /// Children of the component.
    pub(crate) children: Html,
}
