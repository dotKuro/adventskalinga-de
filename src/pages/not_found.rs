use stylist::yew::styled_component;
use stylist::Style;
use yew::prelude::*;

fn container_styles() -> Style {
    Style::new(format!(
        r#"
            display: flex;
            align-items: center;
            justify-content: center;
            width: 100%;
            height: 100%;
        "#
    ))
    .expect("Css string should be correct")
}

#[styled_component]
pub fn NotFound() -> Html {
    html! {
        <div class={container_styles()}>
            {"This Page was not found. 😵‍💫"}
        </div>
    }
}
