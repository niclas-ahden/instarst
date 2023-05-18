use leptos::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1>"Rust bro"</h1>
        <h2>"Management Consulting"</h2>
    }
}
