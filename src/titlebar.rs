#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn TitleBar(cx: Scope) -> Element {
    let title = "title";
    cx.render(rsx! {
        div { 
            id: "title-bar",
            div { 
                id: "title-bar-menu-icon",
                "a"
            }
            div {
                // Potential Icon here.
                id: "title-bar-unused-icon-slot-left",
                class: "hidden",
                "a"
            }
            div { 
                id: "title-bar-title", // Fuck me I have no intelligence.
                "{title}"
            }
            div {
                // Potential Icon here.
                id: "title-bar-unused-icon-slot-right",
                class: "hidden",
                "a"
            }
            div {
                id: "title-bar-settings-icon",
                class: "hidden",
                "s"
            }
         }
    })
}
