use crate::pages::puzzle_page::Color;
use stylist::yew::styled_component;
use stylist::Style;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ColorPickerProps {
    pub name: String,
    pub checked_color: Option<Color>,
    pub on_change: Callback<Color>,
}

fn container_styles() -> Style {
    Style::new(format!(
        r#"
            display: flex;
            flex-direction: row;
            flex-wrap: wrap;
            width: 150px;
            border-style: solid;
            border-width: 3px;
            border-radius: 10px;
            border-color: #274e5c;
            background-color: #d89696;
            padding: 5px;
            margin-left: 50px;
            margin-right: 50px;
            margin-top: 10px;
            margin-bottom: 10px;
        "#
    ))
    .expect("Css string should be correct")
}

fn radio_styles() -> Style {
    Style::new(format!(
        r#"
            display: none;
        "#
    ))
    .expect("Css string should be correct")
}

fn label_styles(checked: bool) -> Style {
    Style::new(format!(
        r#"
            width: 50px;
            height: 50px;
            display: flex;
            justify-content: center;
            align-items: center;
            border-style: solid;
            border-width: 3px;
            border-color: {};
            box-sizing: border-box;
            border-radius: 10px;
            font-size: 35px;
        "#,
        {
            if checked {
                "#fbede4"
            } else {
                "transparent"
            }
        },
    ))
    .expect("Css string should be correct")
}

#[styled_component]
pub fn ColorPicker(props: &ColorPickerProps) -> Html {
    let colors = vec![
        ("🟥", Color::Red),
        ("🟧", Color::Orange),
        ("🟫", Color::Brown),
        ("🟦", Color::Blue),
        ("🟪", Color::Purple),
        ("🟨", Color::Yellow),
        ("🟩", Color::Green),
        ("⬜️", Color::White),
        ("⬛", Color::Black),
    ];
    let color_buttons: Vec<Html> = colors
        .into_iter()
        .map(|(emoji, color)| {
            let checked = Some(color.clone()) == props.checked_color;
            html! {
                    <label
                        class={label_styles(checked)}
                    >
                        <div>
                        { emoji }
                        </div>
                        <input
                            checked={ checked }
                            type="radio"
                            name={props.name.to_string()}
                            class={radio_styles()}
                            onclick={{
                                let on_change = props.on_change.clone();
                                Callback::from(move |_| on_change.clone().emit(color.clone()))
                            }}
                        />
                    </label>
            }
        })
        .collect();

    html! {
        <div class={container_styles()} role="radiogroup">
            { color_buttons }
        </div>
    }
}
