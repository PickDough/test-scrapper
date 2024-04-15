use yew::prelude::*;

#[function_component]
pub fn CardHeader(props: &Props) -> Html {
    html! {
        <div class="card text-bg-primary mb-3">
            <div class="card-body">
                <p class="card-text h5">{ props.children.clone() }</p>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html, // the field name `children` is important!
}
