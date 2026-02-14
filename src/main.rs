use leptos::{logging::log, prelude::*};

fn main() {
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <button
            on:click=move |_| {
                *set_count.write() += 1;
            }
            class:red=move || count.get() % 2 == 1
        >
            "Click me"
        </button>
        <p>"Double count: " {move || count.get() * 2}</p>
        <progress
            max="50"
            value=count
        />
    }
}
