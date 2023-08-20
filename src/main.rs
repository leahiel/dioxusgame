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
mod comp_resourcebar;
use comp_resourcebar::ResourceBarComponent;
mod resource;



// The starting function. This is called first, which then launches the web app.
fn main() {
    dioxus_web::launch(App);
}

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
        div { 
            id: "main-component",
            ActionsComponent {}
        }
        ResourceBarComponent {}
        FootBar {}
    })
}
