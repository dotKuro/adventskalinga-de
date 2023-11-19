use gloo_storage::LocalStorage;
use gloo_storage::Storage;
use std::fmt::Display;
use uuid::Uuid;
use yew::prelude::*;

const SESSION_ID_KEY: &str = "session_id";

#[derive(Clone, PartialEq)]
pub struct SessionId {
    inner: String,
}

impl From<String> for SessionId {
    fn from(value: String) -> Self {
        Self { inner: value }
    }
}

impl Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

#[derive(PartialEq, Properties)]
pub struct SessionIdProviderProps {
    pub children: Children,
}

#[function_component]
pub fn SessionIdProvider(props: &SessionIdProviderProps) -> Html {
    let session_id_result = LocalStorage::get::<String>(SESSION_ID_KEY);
    let session_id: SessionId = match session_id_result {
        Ok(id) => id.into(),
        Err(_) => {
            let id = Uuid::new_v4().to_string();
            LocalStorage::set(SESSION_ID_KEY, id.clone()).unwrap();
            id.into()
        }
    };

    html! {
        <ContextProvider<SessionId> context={session_id}>
            {props.children.clone()}
        </ContextProvider<SessionId>>
    }
}

#[hook]
pub fn use_session_id() -> SessionId {
    use_context::<SessionId>().unwrap()
}
