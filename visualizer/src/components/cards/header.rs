//! Card header component.
use yew::prelude::*;

#[function_component]
pub(crate) fn CardHeader(props: &Props) -> Html {
    html! {
        <div class="card text-bg-primary mb-3">
            <div class="card-body">
                <p class="card-text h5">{ props.children.clone() }</p>
            </div>
        </div>
    }
}

/// Properties for the card header component. Html between this component will
/// be rendered inside.
#[derive(Properties, PartialEq)]
pub(crate) struct Props {
    /// Children of the component.
    pub(crate) children: Html,
}
