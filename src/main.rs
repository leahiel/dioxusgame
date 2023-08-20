#![allow(non_snake_case)]

use dioxus::prelude::*;

mod titlebar;
use titlebar::TitleBar;
mod footbar;
use footbar::FootBar;
mod action;
use action::*;
mod comp_actions;
use comp_actions::ActionsComponent;
mod resource;



// The starting function. This is called first, which then launches the web app.
fn main() {
    dioxus_web::launch(App);
}

// Make Title Bar [Appears on every thing]
// Make Main Page Section [Changes]
    // Make button thing.
    // Make Resource Thing
// Make Footer Bar [Appears on every thing.]

// TODO: state update should only occur n times a second, so make a "update state preview" and then update state with preview every deltatime. Updating state requires re-renders, which is oopsies daisies.
// I will just write_silently(), and then have a coroutine that write()s every n time.

fn App(cx: Scope) -> Element {
    // Initialization
    resource::init_resources(cx);
    action::init_actions(cx);

    // Loop
    // TODO begin loop

    // Render
    cx.render(rsx! {
        // TODO Do include stylesheet properly.
        style { include_str!("assets/css/stylepage.css")}
        TitleBar {}
        div { id: "main-component",
            ActionsComponent {}
        }
        FootBar {}
    })
}
