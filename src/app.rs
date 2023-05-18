use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::routes::{home::*, blog::*, open_positions::*};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/instarst.css"/>

        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous"/>
        <Link href="https://fonts.googleapis.com/css2?family=DM+Serif+Display:ital@0;1&display=swap" rel="stylesheet"/>

        <Title text="instarst"/>

        <Router>
            <nav>
                <ul>
                    <li>
                        <A href="" exact=true>"Home"</A>
                    </li>
                    <li>
                        <A href="/blog">"Blog"</A>
                    </li>
                    <li>
                        <A href="/open-positions">"Open Positions"</A>
                    </li>
                </ul>
            </nav>

            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <Home/> }/>
                    <Route path="/blog" view=|cx| view! { cx, <Blog/> }/>
                    <Route path="/open-positions" view=|cx| view! { cx, <OpenPositions/> }/>
                    <Route path="/open-positions/typescript-front-end-developer" view=|cx| view! { cx, <FrontEndDeveloper/> }/>
                    <Route path="/open-positions/typescript-back-end-developer" view=|cx| view! { cx, <BackEndDeveloper/> }/>
                </Routes>
            </main>

            <footer class="Footer">
                <p class="Footer-copyright">"©️  Rust bros"</p>
            </footer>
        </Router>
    }
}
