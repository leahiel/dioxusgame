#![allow(non_snake_case)]

use dioxus::prelude::*;
use js_sys::Date;

mod titlebar;
use titlebar::TitleBar;
mod footbar;
use footbar::FootBar;
mod action;
use action::*;
mod resource;
use resource::*;
mod comp_actions;
use comp_actions::ActionsComponent;
mod comp_resourcebar;
use comp_resourcebar::ResourceBarComponent;

// The starting function. This is called first, which then launches the web app.
fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    // Initialization
    resource::init_resources(cx);
    action::init_actions(cx);

    // Start gameloop.
    // TODO: Put last_tick in cx.
    // TODO: Or, update all resources here?
    let mut last_tick = Date::now();
    use_future(cx, (), |_| async move {
        loop {
            if Date::now() - last_tick >= 16.0 {
                last_tick = Date::now();
                log::info!("{last_tick}");

                gloo::timers::future::sleep(std::time::Duration::from_millis(16)).await;
            }
        }
    });

    // Initial render.
    cx.render(rsx! {
        // TODO Do include stylesheet properly.
        style { include_str!("assets/css/stylepage.css")}
        TitleBar {}
        div { 
            id: "main-component",
            ActionsComponent {}
            "{last_tick}"
        }
        ResourceBarComponent {}
        FootBar {}
    })
    
}
