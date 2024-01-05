use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use leptos_theme::theme::{ThemeProvider, use_theme};
use leptos_theme::types::{Theme};


#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/demo.css"/>

        // 1. Wrap your project with ThemeProvider
        <ThemeProvider>
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
        <main class="font-robotomono h-screen flex flex-col items-center justify-around bg-[#fdfdfd] dark:bg-[#0a0a0a] dark:text-white w-full">
            <div class="space-y-24">
                <div>
                    <a class=LINK_CSS href=REPO_LINK target="_blank" rel="noreferrer">
                        <p class="text-3xl font-semibold">leptos-theme</p>
                    </a>
                    <br/>
                    <div class="space-y-1">
                        <p>Perfect leptos dark mode in 2 lines of code.</p>
                        <em class="text-sm">
                            "Ok a little more than 2 lines if you want to toggle between themes."
                        </em>
                    </div>
                </div>
                <div class="space-y-20">
                    <div class="space-y-2">
                        <p class="">
                            Current theme: {move || theme_signal.get().to_string()}
                        </p>

                        <div class="flex space-x-4">
                            <p>
                            "Select theme: "
                            </p>
                            {theme_buttons
                                .into_iter()
                                .map(|theme| {
                                    view! {
                                        <button
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
                    <div class="">
                        <div class="p-4 bg-[#f0f0f0] dark:bg-[#131313] hover:cursor-text">
                        <pre>
                            <span>
                                "use "
                                <a href="" class=LINK_CSS target="_blank" rel="noreferrer">
                                    <span class=CRATE_CSS>"leptos_theme"</span>
                                </a>"::theme::"<span class=THEME_CSS>"ThemeProvider"</span>";"
                            </span>
                            <br /> <br />
                            <span class=THEME_CSS>
                                "<ThemeProvider>"
                            </span>
                            <br/>
"   <Router>
        <Routes>
            <Route path=\"\" view=HomePage/>
        </Routes>
    </Router>"<br/>
<span class=THEME_CSS>
    "</ThemeProvider>"
</span>
                        </pre>
                        </div>
                    </div>
                </div>
            </div>

            <div class="flex justify-center space-x-8">
                {
                    footer_links
                        .into_iter()
                        .map(|(text, link)| {
                    view! {
                        <a
                            class=LINK_CSS
                            href=link
                            target="_blank"
                            rel="noreferrer"
                        >
                            {text}
                        </a>
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
        <main class=format!("bg-white dark:bg-[#0a0a0a] dark:text-white text-black h-screen w-full flex flex-col items-center justify-center")>
            <p class="font-robotomono">Unknown command: {unknown}</p>
        </main>
    }
}
