use leptos::*;
mod progress_bar;
use progress_bar::ProgressBar;

#[component]
fn App() -> impl IntoView {
    let (x, set_x) = create_signal(0);

    let double_count = move || x() * 2;
    view! {
        <button on:click=move|_| {
            set_x.update(|n| *n += 10)
        }
        // class:red=move || count() % 2 == 1
        //When dashes and numbers or other characters exists in the class name, use this format
        class=("button-20", move || x() % 2 == 1)
                // and toggle individual CSS properties with `style:`
                style:left=move || format!("{}px", x() + 100)
                style:background-color=move || format!("rgb({}, {}, 100)", x(), 100)
                style:max-width="400px"
                // Set a CSS variable for stylesheet use
                style=("--columns", x)
        >
            "Click me: "
            {x}
        </button>
        <ProgressBar progress=x />
        <br />
        // add a second progress bar
        <ProgressBar progress=Signal::derive(double_count) />
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App /> }
    })
}
