use stylist::yew::styled_component;
use stylist::Style;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct NumberPickerProps {
    pub name: String,
    pub picked_number: i32,
    pub on_change: Callback<i32>,
}

fn input_styles() -> Style {
    Style::new(format!(
        r#"
            width: 150px;
            height: 150px;
            font-size: 80px;
            background-color: #d89696;
            border-style: solid;
            border-width: 3px;
            border-radius: 10px;
            border-color: #274e5c;
            color: #274e5c;
            margin-left: 50px;
            margin-right: 50px;
            margin-top: 10px;
            margin-bottom: 10px;
            text-align: center;

            &::-webkit-outer-spin-button,
            &::-webkit-inner-spin-button {{
                -webkit-appearance: none;
                margin: 0;
            }}

            -moz-appearance: textfield;
        "#
    ))
    .expect("Css string should be correct")
}

#[styled_component]
pub fn NumberPicker(props: &NumberPickerProps) -> Html {
    html! {
        <input
            class={input_styles()}
            type="number"
            name={props.name.clone()}
            min={0}
            max={1_000_000}
            value={props.picked_number.to_string()}
            onchange={{
                let on_change = props.on_change.clone();
                Callback::from(move |event: Event| {
                    let target: Option<EventTarget> = event.target();
                    let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                    if let Some(input) = input {
                        on_change.clone().emit(input.value().parse::<i32>().unwrap())
                    };
                })
            }}
        />
    }
}
