/// This is the main Actions component. Any non-Story related actions should occur here.

use dioxus::prelude::*;
use std::collections::HashMap;
use break_eternity::Decimal;

use crate::Action;
use crate::Resource;

// TODO: Lock out a resource after clicking on it until holding-down-tick-time. Allow holding to proc resource upgrade.

pub fn ActionsComponent(cx: Scope) -> Element {
    // NOTE: You may need to dereference and attach `.read()` or `.write()` to use these two variables.
    // e.g. &*actions.read() -> Readable HashMap of all Actions.
    // If writing to a value in HashMap, you'll need to `.get_mut("value")` it.
    // resources.write().get_mut("value").unwrap().field -> Gives you value.field.
    let actions = use_shared_state::<HashMap<&'static str, Action>>(cx).unwrap();
    let resources = use_shared_state::<HashMap<&'static str, Resource>>(cx).unwrap();
    
    cx.render(rsx! {
        // Render every available Action.
        div {
            for (_, action) in &*actions.read() {
                // if cost resources are met, then
                button {
                    onclick: move |_| {
                        // NOTE: Because the button will be disabled if we don't have enough resources, we don't need to check for that.

                        // COSTS
                        // TODO: Loop through costresources
                        resources.write().get_mut("energy").unwrap().amountcurrent -= 1;

                        // RESULTS
                        // TODO: Loop through resultresources
                        let potential_value = resources.read()["destiny"].amountcurrent + 1;
                        let max_value = resources.read()["destiny"].amountmax;
                        resources.write().get_mut("destiny").unwrap().amountcurrent = Decimal::min(&potential_value, max_value);

                    },
                    "{action.title}: {action.costamounts[0]} {action.costresources[0].name}"
                }
                span {
                    "{action.description}"
                }

                // TODO: if cost resources are not met, then disable button
            }
        }
    })
}
