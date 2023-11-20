pub mod router;
pub mod session_id;

use crate::css::StyleReset;
use router::Router;
use session_id::SessionIdProvider;
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};

    html! {
        <SessionIdProvider>
            <StyleReset />
            <Suspense {fallback}>
                <Router />
            </Suspense>
        </SessionIdProvider>
    }
}
