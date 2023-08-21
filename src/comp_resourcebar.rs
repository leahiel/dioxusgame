/// This is the ResourceBar component.

use dioxus::prelude::*;
use std::collections::HashMap;

use crate::resource::Resource;

pub fn ResourceBarComponent(cx: Scope) -> Element {
    let resources = use_shared_state::<HashMap<&'static str, Resource>>(cx).unwrap().read();

    cx.render(rsx! {
        // Render every available Resource.
        div {
            id: "resource-bar",
            for (_, resource) in &*resources {
                span {
                    class: "resource-bar-resource",
                    span {
                        class: "resource-bar-resource-name",
                        "{resource.name}:"
                    }
                    span {
                        class: "resource-bar-resource-amount",
                        "{resource.amountcurrent.floor()} / {resource.amountmax.floor()}"
                    }
                }
            }
        }
    })
}
