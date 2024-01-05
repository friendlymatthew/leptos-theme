use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use leptos_theme::theme::{ThemeProvider, use_theme};
use leptos_theme::types::{Theme};


#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/demo-pure-css.css"/>

        // 1. Wrap your project with ThemeProvider
        <ThemeProvider
            use_data_attribute= true
        >
            <Router>
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path=":else" view=ErrorPage/>
                </Routes>
            </Router>
        </ThemeProvider>
    }
}

#[component]
fn HomePage() -> impl IntoView {

    const CRATE_CSS: &'static str = "text-teal-800 dark:text-teal-300";
    const THEME_CSS: &'static str = "text-indigo-800 dark:text-indigo-300";
    const LINK_CSS: &'static str = "hover:underline underline-offset-4 w-fit decoration-teal-800 dark:decoration-teal-300";
    const REPO_LINK: &'static str = "https://github.com/friendlymatthew/leptos-theme";

    let footer_links: [(&'static str, String); 2] = [
        ("Contribute", format!("{}", REPO_LINK)),
        ("Leave an issue", format!("{}/issues/new", REPO_LINK))
    ];


    let theme_buttons: [Theme; 3] = [Theme::Light, Theme::Dark, Theme::System];

    // 2. retrieve the theme_signal global state
    let theme_signal = use_theme();

    view! {
        <main style="height: 100vh;">
            <div>
                <div>
                    <a href=REPO_LINK target="_blank" rel="noreferrer">
                        <p style="font-size: 30px">leptos-theme</p>
                    </a>
                    <br/>
                    <div>
                        <p>Perfect leptos dark mode in 2 lines of code.</p>
                        <em>
                            "Ok a little more than 2 lines if you want to toggle between themes."
                        </em>
                    </div>
                </div>
                <div style="padding: 4em 0">
                        <p>
                            Current theme: {move || theme_signal.get().to_string()}
                        </p>

                        <div style="display: flex">
                            <p>
                            "Select theme: "
                            </p>
                            {theme_buttons
                                .into_iter()
                                .map(|theme| {
                                    view! {
                                        <button
                                            style="margin: 0 0.4em"
                                            on:click=move |_| {
                                            theme_signal.set(theme);
                                        }>
                                            <p>{theme.to_string()}</p>
                                        </button>
                                    }
                                })
                                .collect::<Vec<_>>()}
                        </div>
                </div>
            </div>

            <div>
                {
                    footer_links
                        .into_iter()
                        .map(|(text, link)| {
                    view! {
                        <div >
                        <a
                            href=link
                            target="_blank"
                            rel="noreferrer"
                        >
                            {text}
                        </a>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </main>
    }
}

#[component]
fn ErrorPage() -> impl IntoView {
    let params = use_params_map();
    let p_unknown = move || params.with(|p| p.get("else").cloned().unwrap_or_default());

    let unknown = p_unknown();

    view! {
        <main>
            <p class="font-robotomono">Unknown command: {unknown}</p>
        </main>
    }
}
