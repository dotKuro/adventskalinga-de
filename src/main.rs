use advent_calendar::app::Router;
use advent_calendar::app::SessionIdProvider;
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
        <SessionIdProvider>
            <StyleReset />
            <App />
        </SessionIdProvider>
    }
}

fn main() {
    yew::Renderer::<Root>::new().render();
}
