use stylist::yew::styled_component;
use stylist::Style;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct InvisibleTextProps {
    pub name: String,
    pub text: String,
}

fn text_style() -> Style {
    Style::new(format!(
        r#"
        display: none;
        "#
    ))
    .expect("Css string should be correct")
}

#[styled_component]
pub fn InvisibleText(props: &InvisibleTextProps) -> Html {
    html! {
            <p
                class={text_style()}
                name={props.name.clone()}
            >
            { props.text.clone() }
            </p>
    }
}
