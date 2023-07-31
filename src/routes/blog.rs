use leptos::*;

struct Article {
    title: String,
    ingress: String,
}

// local configs = require 'lspconfig.configs'
// local util = require "lspconfig/util"
//
// configs.leptos_language_server = {
//   default_config = {
//     cmd = { 'leptos-language-server' },
//     filetypes = { 'rust' },
//     root_dir = util.root_pattern('Cargo.toml', '.git'),
//   },
// }
//
// local lsp = require('lspconfig')
// lsp.leptos_language_server.setup({
//     on_attach=on_attach,
// })

#[component]
pub fn Blog() -> impl IntoView {
    let articles = vec![
        Article {
            title: "Docker inline caching".to_string(),
            ingress: "Caching dependencies in Docker...".to_string(),
        },
        Article {
            title: "Ruby → Sorbet → Haskell → Rust".to_string(),
            ingress: "When I see a tomato there is much that I can doubt.".to_string(),
        },
        Article {
            title: "Leptos language server in neovim".to_string(),
            ingress: "...".to_string(),
        },
    ];

    view! {
        <ul>
            {articles
                .into_iter()
                .map(|a| {
                    view! {
                        <li>
                            <h3>{a.title}</h3>
                            <p>{a.ingress}</p>
                        </li>
                    }
                })
                .collect_view()}
        </ul>
    }
}
