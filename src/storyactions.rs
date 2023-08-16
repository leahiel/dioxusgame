#![allow(non_snake_case)]

/// This is the main Story Actions component. All Story related actions should occur here.
/// These are typically one-off actions, and they may lock other actions.

use dioxus::prelude::*;
use break_eternity::Decimal;

pub fn ActionsComponent(cx: Scope) -> Element {
    let resources = use_shared_state::<Resources>(cx).unwrap();

}