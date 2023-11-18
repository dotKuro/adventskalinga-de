use stylist::yew::styled_component;
use stylist::Style;
use yew::prelude::*;

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
    html! {
        <div class={container_styles()}>
            <div class={calendar_styles()}/>
        </div>
    }
}
