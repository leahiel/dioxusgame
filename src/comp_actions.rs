/// This is the main Actions component. Any non-Story related actions should occur here.

use dioxus::prelude::*;
use std::collections::HashMap;

use crate::Action;

// TODO: Lock out a resource after clicking on it until holding-down-tick-time. Allow holding to proc resource upgrade.

pub fn ActionsComponent(cx: Scope) -> Element {
    
    let actions = use_shared_state::<HashMap<&'static str, Action>>(cx).unwrap().read();
    
    cx.render(rsx! {
        // Render every available Action.
        for (_, action) in &*actions {
            div {
                "{action.title}"
                br {}
                "{action.costresources[0].name}"
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
