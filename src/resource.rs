// This contains the base information for Resources.

use dioxus::prelude::*;
use std::collections::HashMap;
use break_eternity::Decimal;

#[derive(Copy, Clone)]
pub struct Resource {
    pub name: &'static str,
    pub description: &'static str,
    pub amountcurrent: Decimal,
    pub amountmax: Decimal,
}

impl Default for Resource {
    fn default() -> Self {
        Resource {
            name: "UNNAMED_RESOURCE",
            description: "UNNAMED_RESOURCE_DESCRIPTION",
            amountcurrent: Decimal::from_number(0.0),
            amountmax: Decimal::from_number(10.0),
        }
    }
}


/// Creates a HashMap [key: &'static str, value: Resource]
/// 
/// This HashMap contains every Resource, which is then added to the shared state.
/// 
/// Access with:
/// ```
/// let mut varname = use_shared_state::<HashMap<&'static str, Resource>>(cx).unwrap().read();
/// ```
pub fn init_resources(cx: Scope) {
    // Create HashMap
    let mut resources_hashmap: HashMap<&'static str, Resource> = HashMap::new();

    // Add Resources to HashMap
    resources_hashmap.insert("destiny", Resource { name: "Destiny", amountcurrent: Decimal::from_number(10.0), ..Default::default()});
    resources_hashmap.insert("energy", Resource { name: "Energy", ..Default::default()});

    // Give HashMap to SharedState.
    use_shared_state_provider(cx, || resources_hashmap);
}

// NOTE For updating the game with new/more resources, I will need to find all Resources not in the cx and add them one at a time.
