## 🌗 *leptos-theme*
`leptos-theme` effortlessly toggles between light and dark modes in your leptos application.

## Overview
You can set up dark mode with two lines:

Wrap your project with `<ThemeProvider />`:
```html
view! {
    <Stylesheet id="leptos" href="/pkg/demo.css"/>

    <ThemeProvider>
        <Router>
            <Routes>
                <Route path="/" view=HomePage/>
                <Route path=":else" view=ErrorPage/>
            </Routes>
        </Router>
    </ThemeProvider>
}
```

Now your `HomePage` and `ErrorPage` are dark mode ready! 

Toggle themes on-the-fly with use_theme():
```rust
use leptos_theme::{
    theme::use_theme, 
    types::Theme
};

// inside <HomePage />

let current_theme = use_theme();

view! {
    <button
        on:click=move |_| {
        theme_signal.set(Theme::Light);
    }>
        <p>Light Mode</p>
    </button>
}
```

That's it!


## Features
- Choose between class or data attribute selectors
- Harmonize with system preferences using `prefers-color-scheme` 
- Keep themes consistent across multiple tabs and windows

## Demo

`leptos_theme` supports both class and data attribute selectors.

- For a tailwind + class selector demo: https://leptos-theme.vercel.app/
  - [How to guide](https://github.com/friendlymatthew/leptos-theme/tree/main/example/demo#readme)

- For a pure CSS + data selector demo: https://leptos-theme-data-attribute.vercel.app/
  - [How to guide](https://github.com/friendlymatthew/leptos-theme/tree/main/example/demo-pure-css#readme)


## Contributing
Omg that would be amazing.


## Todo!
- [ ] custom theming
- [ ] force pages to specific themes