use leptos::*;
use leptos_router::*;
mod progress;
use progress::Progress;
mod counter;
use counter::Counter;
mod interation;
use interation::Iteration;

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <nav style="display:flex; gap: 8px;">
                <a href="/">"Home"</a>
                <a href="/progress">"Progress"</a>
                <a href="/counter">"Counter"</a>
                <a href="/iteration">"Iteration"</a>
            </nav>
            <main>
                <Routes>
                    <Route path="/" view=|| view! { <h1>Welcome to Leptos</h1> } />
                    <Route path="/counter" view=Counter />
                    <Route path="/progress" view=Progress />
                    <Route path="/iteration" view=Iteration />
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App /> }
    })
}
