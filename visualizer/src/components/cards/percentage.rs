//! This module contains the percentage card component.
use yew::prelude::*;

#[function_component]
pub(crate) fn CardBodyPercentage(props: &PropsPercentage) -> Html {
    let red = f32::from(
        props
            .percentage
            .to_string()
            .parse::<u8>()
            .unwrap_or_default(),
    ) * 2.55
        * 4.0;
    let green = f32::from(
        100 - props
            .percentage
            .to_string()
            .parse::<u8>()
            .unwrap_or_default(),
    ) * 2.55;
    let classes =
        classes!("card", format!("text-bg-{}", props.style.clone()), "mb-3");
    html! {
        <div class={classes}
            style={format!(
                "background-color: rgb({}, {}, 0) !important;",
                red,
                green
            )}>
            <div class="card-body">
                <p class="card-text">{ props.children.clone() }</p>
            </div>
        </div>
    }
}

/// Properties for the percentage card component.
#[derive(Properties, PartialEq)]
pub(crate) struct PropsPercentage {
    /// The style of the card. Default is `light`. Look at Bootstrap colors for
    /// more information.
    #[prop_or(AttrValue::from("light"))]
    pub(crate) style: AttrValue,
    /// The percentage to display in the card. Higher is more important e.g.
    /// 100 - red, 0 - green.
    #[prop_or(AttrValue::from("100"))]
    pub(crate) percentage: AttrValue,
    /// Children of the component.
    pub(crate) children: Html,
}
