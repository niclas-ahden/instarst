use leptos::*;

struct Article {
    title: String,
    ingress: String,
}

#[component]
pub fn Blog(cx: Scope) -> impl IntoView {
    let articles = vec![
        Article {
            title: "Docker inline caching".to_string(),
            ingress: "Caching dependencies in Docker...".to_string()
        },
        Article {
            title: "Ruby → Sorbet → Haskell → Rust".to_string(),
            ingress: "When I see a tomato there is much that I can doubt.".to_string()
        },
    ];

    view! { cx,
        <ul>
        {articles.into_iter()
            .map(|a| view! { cx,
                <li>
                    <h3>{a.title}</h3>
                    <p>{a.ingress}</p>
                </li>
            })
            .collect_view(cx)}
        </ul>
    }
}
