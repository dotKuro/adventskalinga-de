use advent_calendar::app::Router;
use advent_calendar::css::StyleReset;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <Router />
    }
}

#[function_component]
fn Root() -> Html {
    html! {
        <>
            <StyleReset />
            <App />
        </>
    }
}

fn main() {
    yew::Renderer::<Root>::new().render();
}
