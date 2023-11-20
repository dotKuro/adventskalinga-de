mod door;

use door::Door;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use stylist::yew::styled_component;
use stylist::Style;
use yew::prelude::*;

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

fn calendar_styles() -> Style {
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

#[styled_component]
pub fn Calendar() -> Html {
    let doors_data = use_state(|| {
        (1u8..=24)
            .map(|number| DoorData {
                number,
                open: false,
            })
            .collect::<Vec<DoorData>>()
    });
    let session_id = use_session_id();
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
                <Door number={door_data.number} open={door_data.open} />
            }
        })
        .collect::<Vec<Html>>();

    html! {
        <div class={container_styles()}>
            <div class={calendar_styles()}>
                { doors }
            </div>
        </div>
    }
}
