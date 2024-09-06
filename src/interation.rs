use leptos::*;
#[component]
fn StaticIteration(length: usize) -> impl IntoView {
    let counter_buttons = (1..=length)
        .map(|x| create_signal(x))
        .map(|(count, set_count)| {
            view! {
                <li>
                    <button on:click=move |_| {
                        set_count.update(|n| *n += 1);
                    }>{count}</button>
                </li>
            }
        })
        .collect_view();

    view! {
        <h2>Static Iteration</h2>
        <ul>{counter_buttons}</ul>
    }
}
/*
set_counters
.update(|counters| {
    counters.retain(|(counter_id, _)| counter_id != &id)
})
is siimilar as
setCounter(prev => prev.filter(counterId => counterId !== id)) in react
*/

#[component]
fn DynamicIteration(initial_length: usize) -> impl IntoView {
    let mut next_counter_id = initial_length;
    let initial_counters = (0..initial_length)
        .map(|id| (id, create_signal(0)))
        .collect::<Vec<_>>();
    let (counters, set_counters) = create_signal(initial_counters);
    let add_counter = move |_| {
        let sig = create_signal(next_counter_id + 1);
        set_counters.update(move |counters| counters.push((next_counter_id, sig)));
        next_counter_id += 1;
    };
    view! {
        <h2>Dynamic Iteration</h2>
        <div>
            <button on:click=add_counter>Add counter</button>
            <ul>
                <For
                    each=counters
                    key=|counter| counter.0
                    children=move |(id, (count, set_count))| {
                        view! {
                            <li>
                                <p>id: {id}</p>
                                <button on:click=move |_| {
                                    set_count.update(|n| *n += 1)
                                }>Current count: {count}</button>
                                <button on:click=move |_| {
                                    set_counters
                                        .update(|counters| {
                                            counters.retain(|(counter_id, _)| counter_id != &id)
                                        })
                                }>Remove</button>
                            </li>
                        }
                    }
                ></For>
            </ul>

        </div>
    }
}
#[component]
pub fn Iteration() -> impl IntoView {
    view! {
        <h1>Iteration</h1>
        <StaticIteration length=5 />
        <DynamicIteration initial_length=5 />
    }
}
