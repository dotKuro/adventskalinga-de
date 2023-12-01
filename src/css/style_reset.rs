use stylist::yew::{styled_component, Global};
use yew::prelude::*;

#[styled_component]
pub fn StyleReset() -> Html {
    html! {
        <Global css={
            css!(r#"
            html, body {
                margin: 0;
                height: 100%;
                color: #274e5c;
            }
            a {
                text-decoration: none;
            }
            "#)
        }/>
    }
}
