#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn FootBar(cx: Scope) -> Element {
    cx.render(rsx! {
        div { 
            id: "foot-bar",
            div {
                // Potential Icon here.
                id: "foot-bar-unused-icon-slot-1",
                class: "hidden foot-bar-icon",
                "a"
            }
            div {
                // Potential Icon here.
                id: "foot-bar-unused-icon-slot-2",
                class: "hidden foot-bar-icon",
                "a"
            }
            div {
                // Potential Icon here.
                id: "foot-bar-unused-icon-slot-3",
                class: "hidden foot-bar-icon",
                "a"
            }
            div {
                // Potential Icon here.
                id: "foot-bar-unused-icon-slot-4",
                class: "hidden foot-bar-icon",
                "a"
            }
            div {
                // Potential Icon here.
                id: "foot-bar-unused-icon-slot-5",
                class: "hidden foot-bar-icon",
                "a"
            }
            div {
                // Potential Icon here.
                id: "foot-bar-unused-icon-slot-6",
                class: "hidden foot-bar-icon",
                "a"
            }
            div {
                // Potential Icon here.
                id: "foot-bar-unused-icon-slot-7",
                class: "hidden foot-bar-icon",
                "a"
            }
        }
    })
}
