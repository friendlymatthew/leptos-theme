## *demo*

This is a demo using pure CSS and data selectors.

Here's a live version: https://leptos-theme-data-attribute.vercel.app/


## Quickstart
1. To set up `leptos-theme`, install the dependency
```shell
cargo install leptos-theme
```

2. Wrap your project with `<ThemeProvider />` and specify `use_data_attribute` as `true`.

```html
view! {
    <Stylesheet id="leptos" href="/pkg/demo.css"/>

    <ThemeProvider
        use_data_attribute=true
    >
        <Router>
            <Routes>
                <Route path="/" view=HomePage/>
                <Route path=":else" view=ErrorPage/>
            </Routes>
        </Router>
    </ThemeProvider>
}
```

3. Update your styles to include dark mode with the `[data-theme="dark"]: 

```css
* {
    margin: 0;
    padding: 0;
    border: 0;
}

html, body {
    font-family: "Roboto Mono",monospace;
    color: #000;
}

main {
    display: flex;
    flex-direction: column;
    justify-content: space-around;
    padding: 0 3em;
    background-color: #fdfdfd;
}

*[data-theme="dark"], html[data-theme='dark'] main, body[data-theme='dark'] main {
    background-color: #0a0a0a;
    color: #fff;
}
```

4. You're all set!

```shell
trunk serve --open
```
