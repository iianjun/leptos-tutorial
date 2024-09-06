use leptos::*;

#[component]
pub fn Counter() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <h1>Counter</h1>
        <button on:click=move |_| { set_count.update(|n| *n += 1) }>"Click me: " {count}</button>
    }
}
