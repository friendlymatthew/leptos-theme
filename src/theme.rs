use crate::types::Theme;
use leptos::*;
use leptos_use::storage::{use_local_storage, JsonCodec};
use leptos_use::use_preferred_dark;

/// Define a constant for the local storage key used to store the theme setting.
const STORAGE_KEY: &'static str = "theme";

/// Updates the class selector for the respective theme.
/// This function is responsible for applying the correct CSS class to the HTML and body elements based on the current theme.
///
/// ## Arguments
/// * `theme` - The current theme (Light, Dark, System)
/// * `prefers_dark` - Boolean flag indicating whether the system prefers a dark theme.
fn update_css_for_theme(theme: Theme, prefers_dark: bool, use_data_attribute: bool) {
    let document = web_sys::window().unwrap().document().unwrap();
    let html_element = document.document_element().unwrap();

    if use_data_attribute {
        match theme {
            Theme::Light => {
                html_element.set_attribute("data-theme", "light").unwrap();
            }
            Theme::Dark => {
                html_element.set_attribute("data-theme", "dark").unwrap();
            }
            Theme::System => match prefers_dark {
                true => html_element.set_attribute("data-theme", "dark").unwrap(),
                false => html_element.set_attribute("data-theme", "light").unwrap(),
            },
        }
    } else {
        match theme {
            Theme::Light => {
                html_element.class_list().remove_1("dark").unwrap();
            }
            Theme::Dark => {
                html_element.class_list().add_1("dark").unwrap();
            }
            Theme::System => match prefers_dark {
                true => html_element.class_list().add_1("dark").unwrap(),
                false => html_element.class_list().remove_1("dark").unwrap(),
            },
        }
    }
}

/// Provides the global `Theme` state
///
/// This function is used to access the current theme state from the global context.
/// The state is wrapped as an `RwSignal`.
pub fn use_theme() -> RwSignal<Theme> {
    use_context::<RwSignal<Theme>>().expect("there should be a global theme state")
}

/// The `ThemeProvider` component.
///
/// This component provides a theme context to its children, allowing them to access and react to theme changes.
///
/// ## Properties
/// * `enable_system` - A boolean flag to sync with the system theme preference.
///                     Defaults to `true`.
/// * `children` - The child components that will consume the theme context.
#[component]
pub fn ThemeProvider(
    #[prop(default = false)] use_data_attribute: bool,
    #[prop(default = true)] enable_system: bool,
    children: Children,
) -> impl IntoView {
    let prefers_dark = enable_system && use_preferred_dark().get();

    // Attempt to retrieve the theme from local storage
    let (theme_storage_state, set_theme_storage_state, _) =
        use_local_storage::<Theme, JsonCodec>(STORAGE_KEY);

    // Determine the initial theme from local storage
    let initial_theme = theme_storage_state.get();

    // Initialize the theme state with the determined initial theme
    let theme_state = RwSignal::new(initial_theme);
    provide_context(theme_state.clone());

    // Update local storage whenever the theme state changes
    create_effect(move |_| {
        let current_theme = theme_state.get();
        set_theme_storage_state.set(current_theme.clone());
        update_css_for_theme(current_theme, prefers_dark, use_data_attribute)
    });

    view! {
        {children()}
    }
}
