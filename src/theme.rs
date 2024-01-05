use leptos::*;
use leptos_use::storage::{use_local_storage, JsonCodec};
use leptos_use::use_preferred_dark;
use serde::{Deserialize, Serialize};

const STORAGE_KEY: &'static str = "theme";

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    System
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Dark
    }
}

impl Theme {
    pub fn to_string(self) -> String {
        String::from(match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
            Theme::System => "system",
        })
    }

}

fn update_css_for_theme(theme: Theme, prefers_dark: bool) {
    let document = web_sys::window().unwrap().document().unwrap();
    let html_element = document.document_element().unwrap();

    match theme {
        Theme::Light => {
            html_element.class_list().remove_1("dark").unwrap();
        },
        Theme::Dark => {
            html_element.class_list().add_1("dark").unwrap();
        },
        Theme::System => {
            if prefers_dark {
                html_element.class_list().add_1("dark").unwrap();
            } else {
                html_element.class_list().remove_1("dark").unwrap();
            }
        }
    }
}

#[component]
pub fn ThemeProvider(
    /// children components
    children: Children
) -> impl IntoView {
    let prefers_dark = use_preferred_dark().get();

    // Attempt to retrieve the theme from local storage
    let (theme_storage_state, set_theme_storage_state, _) =
        use_local_storage::<Theme, JsonCodec>(STORAGE_KEY);

    // Determine the initial theme: either from local storage or via `determine_theme`
    let initial_theme = theme_storage_state.get();

    // Initialize the theme state with the determined initial theme
    let theme_state = RwSignal::new(initial_theme);
    provide_context(theme_state.clone());

    // Update local storage whenever the theme state changes
    create_effect(move |_| {
        let current_theme = theme_state.get();
        set_theme_storage_state.set(current_theme.clone());
        // update css
        update_css_for_theme(current_theme, prefers_dark)
    });

    view! {{children()}}
}