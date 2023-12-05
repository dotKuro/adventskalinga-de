mod control;
mod controls;
mod puzzle;

use crate::app::session_id::use_session_id;
use gloo_net::http::Request;
use puzzle::Puzzle;
use serde::{Deserialize, Serialize};
use stylist::yew::styled_component;
use stylist::Style;
use yew::prelude::*;
use yew_hooks::use_window_size;

#[derive(Deserialize, Clone)]
#[serde(tag = "type")]
enum GetPuzzleResponse {
    Success { puzzle: PuzzleData },
    Error { error: String },
}

#[derive(Deserialize, Clone, PartialEq)]
struct PuzzleData {
    pub description: PuzzleDescription,
    pub controls: Vec<PuzzleControl>,
}

#[derive(Deserialize, Clone, PartialEq)]
#[serde(tag = "type", content = "value")]
enum PuzzleDescription {
    Text(String),
    Image(String),
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
#[serde(tag = "type", content = "value")]
enum PuzzleControl {
    ColorPicker(Option<Color>),
    NumberPicker(i32),
    TextInput(String),
    TextSelection(String, Vec<String>),
    InvisibleText(String),
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
#[serde(tag = "type")]
enum Color {
    Red,
    Orange,
    Brown,
    Blue,
    Purple,
    Yellow,
    Green,
    White,
    Black,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
#[serde(tag = "type", content = "value")]
enum Guess {
    Correct { code: String },
    Wrong,
    Error { error: String },
}

#[derive(Serialize)]
struct GetPuzzleBody {
    session_id: String,
    number: u8,
}

#[derive(Serialize, Clone)]
struct GuessAnswerBody {
    session_id: String,
    number: u8,
    answer: Vec<PuzzleControl>,
}

fn container_styles() -> Style {
    Style::new(format!(
        r#"
            display: flex;
            align-items: center;
            justify-content: center;
            width: 100%;
            height: 100%;
            background-image: url('/assests/images/repeating_background.avif');
        "#
    ))
    .expect("Css string should be correct")
}

fn large_puzzle_styles() -> Style {
    Style::new(format!(
        r#"
            display: flex;
            flex-direction: column;
            align-items: center;
            width: 1000px;
            height: 1200px;
            background-color: rgba(255, 255, 255, 0.9);
            box-shadow: 3px 3px 15px grey;
        "#
    ))
    .expect("Css string should be correct")
}

fn small_puzzle_styles() -> Style {
    Style::new(format!(
        r#"
            width: 100%;
            height: 100%;
            overflow-y: scroll;
            background-color: rgba(255, 255, 255, 0.9);
        "#
    ))
    .expect("Css string should be correct")
}

fn title_styles() -> Style {
    Style::new(format!(
        r#"
            font-size: 80px;
            text-align: center;
        "#
    ))
    .expect("Css string should be correct")
}

fn body_styles() -> Style {
    Style::new(format!(
        r#"
            font-size: 40px;
            flex-grow: 1;
        "#
    ))
    .expect("Css string should be correct")
}

fn error_styles() -> Style {
    Style::new(format!(
        r#"
            text-align: center;
        "#
    ))
    .expect("Css string should be correct")
}

#[derive(Properties, PartialEq, Clone)]
pub struct PuzzlePageProps {
    pub number: u8,
}

#[styled_component]
pub fn PuzzlePage(props: &PuzzlePageProps) -> Html {
    let (width, _) = use_window_size();
    let puzzle_data = use_state(|| GetPuzzleResponse::Error {
        error: String::from("Es lädt noch..."),
    });
    let last_guess = use_state(|| None::<Guess>);
    let session_id = use_session_id();
    {
        let puzzle_data = puzzle_data.clone();
        let session_id = session_id.to_string();
        let request_body = GetPuzzleBody {
            session_id,
            number: props.number,
        };
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let new_response: GetPuzzleResponse = Request::post("/api/get-puzzle")
                    .json(&request_body)
                    .unwrap()
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                puzzle_data.set(new_response);
            })
        })
    }

    let handle_change = {
        let puzzle_data = puzzle_data.clone();
        Callback::from(
            move |new_controls: Vec<PuzzleControl>| match (*puzzle_data).clone() {
                GetPuzzleResponse::Success { puzzle } => {
                    puzzle_data.set(GetPuzzleResponse::Success {
                        puzzle: PuzzleData {
                            description: puzzle.description,
                            controls: new_controls,
                        },
                    })
                }
                GetPuzzleResponse::Error { error: _ } => (),
            },
        )
    };

    let handle_submit = {
        let number = props.number;
        let puzzle_data = puzzle_data.clone();
        let last_guess = last_guess.clone();
        Callback::from(move |_| {
            let last_guess = last_guess.clone();
            let answer = match (*puzzle_data).clone() {
                GetPuzzleResponse::Success { puzzle } => puzzle.clone().controls,
                GetPuzzleResponse::Error { error: _ } => Vec::new(),
            };
            let session_id = session_id.to_string();
            let request_body = GuessAnswerBody {
                session_id,
                number,
                answer,
            };
            wasm_bindgen_futures::spawn_local(async move {
                let guess: Guess = Request::post("/api/guess-answer")
                    .json(&request_body)
                    .unwrap()
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                last_guess.set(Some(guess));
            })
        })
    };

    let puzzle_styles = if width >= 1800.0 {
        large_puzzle_styles()
    } else {
        small_puzzle_styles()
    };

    html! {
        <div class={container_styles()}>
            <div class={puzzle_styles}>
                <h1 class={title_styles()}>
                    { format!("Tag {}", props.number) }
                </h1>
                <div class={body_styles()}>
                    { match (*puzzle_data).clone() {
                        GetPuzzleResponse::Success { puzzle } => html! {
                            <Puzzle
                                puzzle={puzzle}
                                last_guess={(*last_guess).clone()}
                                on_change={handle_change}
                                on_submit={handle_submit}
                                />
                        },
                        GetPuzzleResponse::Error { error } => html! {
                            <div class={error_styles()}>
                                { error }
                            </div>
                        },
                    } }
                </div>
            </div>
        </div>
    }
}
