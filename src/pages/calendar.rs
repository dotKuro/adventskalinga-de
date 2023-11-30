mod door;

use chrono::{DateTime, Local};
use door::Door;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use stylist::yew::styled_component;
use stylist::Style;
use yew::prelude::*;
use yew_hooks::use_window_size;

use crate::app::session_id::use_session_id;

#[derive(Deserialize)]
struct DoorData {
    number: u8,
    open: bool,
}

#[derive(Serialize)]
struct ListDoorsBody {
    session_id: String,
}

fn get_date_for_number(number: u8) -> DateTime<Local> {
    let date_string = format!("2023-12-{:0>2}T00:00:00+01:00", number);
    DateTime::parse_from_rfc3339(&date_string).unwrap().into()
}

fn container_styles() -> Style {
    Style::new(format!(
        r#"
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            width: 100%;
            height: 100%;
            background-image: url('/assests/images/repeating_background.avif');
        "#
    ))
    .expect("Css string should be correct")
}
fn title_styles() -> Style {
    Style::new(format!(
        r#"
            font-size: 80px;
        "#
    ))
    .expect("Css string should be correct")
}

fn large_calendar_styles() -> Style {
    Style::new(format!(
        r#"
            display: flex;
            flex-wrap: wrap;
            justify-content: space-evenly;
            align-items: center;
            width: 1600px;
            height: 900px;
            background-image: url('/assests/images/cat_background.webp');
            box-shadow: 3px 3px 15px grey;
        "#
    ))
    .expect("Css string should be correct")
}
fn small_calendar_styles() -> Style {
    Style::new(format!(
        r#"
            width: 100%;
            height: 100%;
            overflow-y: scroll;
        "#
    ))
    .expect("Css string should be correct")
}

#[styled_component]
pub fn Calendar() -> Html {
    let (width, _) = use_window_size();
    let doors_data = use_state(|| {
        (1u8..=24)
            .map(|number| DoorData {
                number,
                open: false,
            })
            .collect::<Vec<DoorData>>()
    });
    let session_id = use_session_id();
    let today = Local::now();
    {
        let doors_data = doors_data.clone();
        let session_id = session_id.to_string();
        let request_body = ListDoorsBody { session_id };
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let new_door_data: Vec<DoorData> = Request::post("/api/list-doors")
                    .json(&request_body)
                    .unwrap()
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                doors_data.set(new_door_data);
            })
        })
    }
    let doors = doors_data
        .iter()
        .map(|door_data| {
            html! {
                <Door
                    number={door_data.number}
                    open={door_data.open}
                    active={today >= get_date_for_number(door_data.number)}
                />
            }
        })
        .collect::<Vec<Html>>();

    let calendar_styles = if width >= 1800.0 {
        large_calendar_styles()
    } else {
        small_calendar_styles()
    };

    html! {
        <div class={container_styles()}>
            <h1 class={title_styles()}>
            {"Ingas Adventskalendar"}
            </h1>
            <div class={calendar_styles}>
                { doors }
            </div>
        </div>
    }
}
