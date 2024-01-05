## 🌗 *leptos-theme*
`leptos-theme` effortlessly toggles between light and dark modes in your leptos application.

<img width="629" alt="Screen Shot 2024-01-05 at 3 58 18 PM" src="https://github.com/friendlymatthew/leptos-theme/assets/38759997/a9393259-1f3e-4570-8b3e-f9e1d95ff2e6">

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

