use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    html! {
        <div class="columns">
            <div class="column">{"First column"}</div>
            <div class="column">{"Second column"}</div>
            <div class="column">{"Third column"}</div>
            <div class="column">{"Fourth column"}</div>
        </div>
    }
}
