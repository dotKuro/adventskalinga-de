use crate::pages::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[at("/")]
    Calendar,
    #[at("/puzzle/:number")]
    Puzzle { number: u8 },
    #[at("/*")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Calendar => html! { <Calendar /> },
        Route::Puzzle { number } => {
            if number <= 24 {
                html! { <PuzzlePage number={number} /> }
            } else {
                html! { <NotFound /> }
            }
        }
        Route::NotFound => html! { <NotFound /> },
    }
}

#[function_component]
pub fn Router() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch}/>
        </BrowserRouter>
    }
}
