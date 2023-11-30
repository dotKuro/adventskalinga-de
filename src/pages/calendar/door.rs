use stylist::yew::styled_component;
use stylist::Style;
use yew::prelude::*;

fn container_styles() -> Style {
    Style::new(format!(
        r#"
            box-sizing: border-box;
            width: 180px;
            height: 180px;
            padding: 4px;
            border-style: solid;
            border-width: 2px;
            border-color: white;
            color: white;
            font-weight: bold;
            font-size: 24px;
            text-shadow: -1px 0 black, 0 1px black, 1px 0 black, 0 -1px black;
            position: relative;
        "#,
    ))
    .expect("Css string should be correct")
}

fn background_style(open: bool) -> Style {
    Style::new(format!(
        r#"
            box-sizing: content-box;
            width: 176px;
            height: 176px;
            position: absolute;
            top: 0;
            left: 0;
            background-color: rgba(200, 200, 200, 0.8);
            opacity: {};
        "#,
        match open {
            true => "100%",
            false => "0",
        }
    ))
    .expect("Css string should be correct")
}

fn number_styles() -> Style {
    Style::new(format!(
        r#"
            position: relative;
            z-index: 1;
        "#,
    ))
    .expect("Css string should be correct")
}
#[derive(Properties, PartialEq, Clone)]
pub struct ContainerProps {
    pub number: u8,
    pub active: bool,
    pub children: Children,
}

#[styled_component]
fn Container(props: &ContainerProps) -> Html {
    if props.active {
        html! {
        <a class={container_styles()} href={format!("/puzzle/{}", props.number)}>
            { props.children.clone() }
        </a>
        }
    } else {
        html! {
        <div class={container_styles()}>
            { props.children.clone() }
        </div>
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct DoorProps {
    pub number: u8,
    pub open: bool,
    pub active: bool,
}

#[styled_component]
pub fn Door(props: &DoorProps) -> Html {
    html! {
        <Container number={props.number} active={props.active}>
            <img
                src={format!("/assests/images/otter_{}.png", props.number)}
                class={background_style(props.open)}
            />
            <div class={number_styles()}>
                {props.number}
            </div>
        </Container>
    }
}
