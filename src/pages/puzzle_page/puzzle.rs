use super::control::Control;
use super::{Guess, PuzzleControl, PuzzleData, PuzzleDescription};
use crate::utils::ignore_in;
use stylist::yew::styled_component;
use stylist::Style;
use yew::prelude::*;

fn container_styles() -> Style {
    Style::new(format!(
        r#"
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: space-between;
            height: 100%;
        "#
    ))
    .expect("Css string should be correct")
}

fn image_styles() -> Style {
    Style::new(format!(
        r#"
            height: 200px;
        "#
    ))
    .expect("Css string should be correct")
}

fn controls_styles() -> Style {
    Style::new(format!(
        r#"
            display: flex;
            flex-direction: row;
            flex-wrap: wrap;
            justify-content: space-evenly;
        "#
    ))
    .expect("Css string should be correct")
}

fn button_styles() -> Style {
    Style::new(format!(
        r#"
            font-size: 40px;
            margin: 5px;
            border-style: solid;
            border-color: transparent;
            border-width: 3px;
            border-radius: 15px;
            padding: 10px;
            background-color: #d89696;

            :active {{
                background-color: #d88888;
            }}
            :hover {{
                border-color: black;
            }}
        "#
    ))
    .expect("Css string should be correct")
}

fn hint_styles() -> Style {
    Style::new(format!(
        r#"
            display: flex;
            flex-direction: column;
            margin: 30px;
            align-items: center;
        "#
    ))
    .expect("Css string should be correct")
}

#[derive(Properties, PartialEq, Clone)]
pub struct PuzzleProps {
    pub puzzle: PuzzleData,
    pub last_guess: Option<Guess>,
    pub on_submit: Callback<()>,
    pub on_change: Callback<Vec<PuzzleControl>>,
}

#[styled_component]
pub fn Puzzle(props: &PuzzleProps) -> Html {
    let description = match props.puzzle.description.clone() {
        PuzzleDescription::Image(src) => {
            html! {
                <img class={image_styles()} src={src}/>
            }
        }
        PuzzleDescription::Text(text) => {
            html! {
                { text }
            }
        }
    };

    fn handle_change(
        index: usize,
        current_controls: Vec<PuzzleControl>,
        on_change: Callback<Vec<PuzzleControl>>,
    ) -> Callback<PuzzleControl> {
        Callback::from(move |new_control: PuzzleControl| {
            let mut current_controls = current_controls.clone();
            current_controls[index] = new_control;
            on_change.emit(current_controls);
        })
    }

    let controls: Vec<Html> = props
        .puzzle
        .controls
        .iter()
        .enumerate()
        .map(|(i, control)| {
            let name = format!("control-{}", i);
            html! {
                <Control
                    control={control.clone()}
                    name={name.clone()}
                    on_change={handle_change(i, props.puzzle.controls.clone(), props.on_change.clone())}
                />
            }
        })
        .collect();

    let submit_button = html! {
        <button
            class={button_styles()}
            onclick={ignore_in(props.on_submit.clone())}
        >
            { "Absenden" }
        </button>
    };

    let guess_hint_and_button = match props.last_guess.clone() {
        Some(guess) => match guess {
            Guess::Correct { code } => html! {
                <>
                    <div class={css!("visibility: hidden;")} aria-hidden="true">
                        { submit_button }
                    </div>
                    <div>
                        { format!("Richtig! Dein Code heute ist: {}", code) }
                    </div>
                </>
            },
            Guess::Wrong => html! {
                <>
                    <div>
                    { submit_button }
                    </div>
                    <div>
                    { "Falsch. Probier nochmal." }
                    </div>
                </>
            },
            Guess::Error { error } => html! {
                <>
                    { submit_button }
                    <div>
                    { error }
                    </div>
                </>
            },
        },
        None => html! {
            <>
            <div>
                { submit_button }
            </div>
            <div class={css!("visibility: hidden;")} aria-hidden="true">
                { "Das hier sieht man nicht" }
            </div>
            </>
        },
    };

    html! {
        <div class={container_styles()}>
            <div>
                { description }
            </div>
            <div class={controls_styles()}>
                { controls }
            </div>
            <div class={hint_styles()}>
                { guess_hint_and_button }
            </div>
        </div>
    }
}
