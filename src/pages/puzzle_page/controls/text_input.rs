use stylist::yew::styled_component;
use stylist::Style;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct TextInputProps {
    pub name: String,
    pub selected_text: String,
    pub on_change: Callback<String>,
}

fn input_styles() -> Style {
    Style::new(format!(
        r#"
            height: 150px;
            font-size: 60px;
            text-align: center;
            background-color: #d89696;
            border-style: solid;
            border-width: 3px;
            border-radius: 10px;
            border-color: #274e5c;
            color: #274e5c;
            margin-top: 20px;
            margin-bottom: 20px;
            padding: 10px;
        "#
    ))
    .expect("Css string should be correct")
}

#[styled_component]
pub fn TextInput(props: &TextInputProps) -> Html {
    html! {
        <>
            <input
                class={input_styles()}
                type="text"
                name={props.name.clone()}
                value={props.selected_text.clone()}
                onchange={{
                    let on_change = props.on_change.clone();
                    Callback::from(move |event: Event| {
                        let target: Option<EventTarget> = event.target();
                        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                        if let Some(input) = input {
                            on_change.clone().emit(input.value())
                        };
                    })
                }}
            />
        </>
    }
}
