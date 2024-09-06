use leptos::*;
/// Shows progress toward a goal.
#[component]
fn ProgressBar(
    /// The maximum value of the progress bar.
    #[prop(default = 100)]
    max: u16,
    /// How much progress should be displayed.
    #[prop(into)]
    progress: Signal<i32>,
) -> impl IntoView {
    view! { <progress max=max value=progress /> }
}

#[component]
pub fn Progress() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;
    view! {
        <h1>Progress bar using props</h1>
        <button style="margin-bottom: 4px;" on:click=move |_| set_count.update(|n| *n = 0)>
            Reset
        </button>
        <button
            style="display: block"
            on:click=move |_| {
                set_count.update(|n| *n += 10);
            }
        >
            Increase count by 10
        </button>
        <p>Current count: {count}</p>
        <ProgressBar progress=count />
        <h2>Double count progress</h2>
        <ProgressBar progress=Signal::derive(double_count) />
    }
}
