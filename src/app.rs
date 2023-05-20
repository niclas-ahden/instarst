use crate::routes::{
    blog::*,
    home::*,
    recruiting::*,
    recruiting::front_end::*,
    recruiting::back_end::*,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

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
        <Link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=DM+Serif+Display&family=Inter:wght@400;700&display=swap"/>

        <Title text="Instateam"/>

        <Router>
            <nav class="MainNav">
                <ul>
                    <li>
                        <A href="" exact=true>"Home"</A>
                    </li>
                    <li>
                        <A href="/recruiting">"Recruiting"</A>
                    </li>
                    // <li>
                    //     <A href="/blog">"Blog"</A>
                    // </li>
                </ul>
            </nav>

            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <Home/> }/>
                    <Route path="/blog" view=|cx| view! { cx, <Blog/> }/>
                    <Route path="/recruiting" view=|cx| view! { cx, <OpenPositions/> }/>
                    <Route path="/recruiting/typescript-front-end-developer" view=|cx| view! { cx, <FrontEndDeveloper/> }/>
                    <Route path="/recruiting/typescript-back-end-developer" view=|cx| view! { cx, <BackEndDeveloper/> }/>
                    // TODO: Implement 404. What's below doesn't work, because it'll catch static
                    // asset paths such as /pkg/instarst.css
                    // <Route path="/*any" view=|cx| view! { cx, <FourOhFour/> }/>
                </Routes>
            </main>

            <footer class="Footer">
                <p class="Footer-copyright">"© 2023 Instateam AB"</p>
            </footer>
        </Router>
    }
}
