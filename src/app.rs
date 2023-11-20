pub mod router;
pub mod session_id;

use crate::css::StyleReset;
use router::Router;
use session_id::SessionIdProvider;
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    html! {
        <SessionIdProvider>
            <StyleReset />
            <Router />
        </SessionIdProvider>
    }
}
