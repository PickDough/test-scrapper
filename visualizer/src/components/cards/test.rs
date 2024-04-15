use yew::prelude::*;

#[function_component]
pub fn CardBodyTest(props: &PropsTest) -> Html {
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

#[derive(Properties, PartialEq)]
pub struct PropsTest {
    #[prop_or(AttrValue::from("light"))]
    pub style: AttrValue,
    pub children: Html,
}
