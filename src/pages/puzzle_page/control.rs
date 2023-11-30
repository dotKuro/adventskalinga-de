use crate::pages::puzzle_page::Color;

use super::controls::ColorPicker;
use super::PuzzleControl;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ControlProps {
    pub control: PuzzleControl,
    pub name: String,
    pub on_change: Callback<PuzzleControl>,
}

#[function_component]
pub fn Control(props: &ControlProps) -> Html {
    match props.control.clone() {
        PuzzleControl::ColorPicker(color) => html! {
            <ColorPicker
                name={props.name.clone()}
                checked_color={color}
                on_change={{
                    let on_change = props.on_change.clone();
                    Callback::from(move |color: Color| on_change.emit(PuzzleControl::ColorPicker(Some(color))))
                }}
            />
        },
        PuzzleControl::NumberPicker => html! {
            <div>
                { "NumberPicker" }
            </div>
        },
    }
}
