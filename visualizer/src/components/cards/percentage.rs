use yew::prelude::*;

#[function_component]
pub fn CardBodyPercentage(props: &PropsPercentage) -> Html {
    let red =
        props.progress.to_string().parse::<u16>().unwrap() as f32 * 2.55 * 4.0;
    let green = ((100 - props.progress.to_string().parse::<u16>().unwrap())
        as f32)
        * 2.55;
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

#[derive(Properties, PartialEq)]
pub struct PropsPercentage {
    #[prop_or(AttrValue::from("light"))]
    pub style: AttrValue,
    #[prop_or(AttrValue::from("100"))]
    pub progress: AttrValue,
    pub children: Html,
}
