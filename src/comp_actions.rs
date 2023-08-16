#![allow(non_snake_case)]

/// This is the main Actions component. Any non-Story related actions should occur here.
/// Actions are generally always available.

use dioxus::prelude::*;
use break_eternity::Decimal;

use crate::Action;
use crate::Resources;

// TODO: Lock out a resource after clicking on it until holding-down-tick-time. Allow holding to proc resource upgrade.

pub fn ActionsComponent(cx: Scope) -> Element {
    
    let actions = use_shared_state::<Vec<Action>>(cx).unwrap();
    let resources = use_shared_state::<Resources>(cx).unwrap();

    cx.render(rsx! {

        // for action in cx.Actions, render
        for action in &*actions.read() {
            div {
                "{action.title}"
            }
        }

        // div {
        //     button {
        //         onclick: move |_| {
        //             if resources.read().energy < resources.read().energyMax {
        //                 let potential_value = resources.read().energy + 1;
        //                 let max_value = resources.read().energyMax;
        //                 resources.write().energy = Decimal::min(&potential_value, max_value);
        //             }
        //         },
        //         "update energy ",
        //     }, 
        //     "energy counter: {resources.read().energy} / {resources.read().energyMax}",
        // }
        // div {
        //     button {
        //         onclick: move |_| {
        //             resources.write().destiny += 1;
        //         },
        //         "update destiny ",
        //     }, 
        //     "destiny counter: {resources.read().destiny}",
        // }
    })
}
