use yew::Callback;

pub fn ignore_in<T>(cb: Callback<()>) -> Callback<T> {
    Callback::from(move |_| cb.emit(()))
}
