#![allow(non_snake_case)]

/// This is all the information for Actions.

use dioxus::prelude::*;
use break_eternity::Decimal;

// NYI: Resources not currently set up to be able to do this.
// use crate::Resources;

pub struct Action {
    // Able to be clicked
    pub available: bool,

    // Story Action, already completed.
    pub completed: bool,

    // Denotes Story Action or not.
    pub completable: bool,

    // Does the user want to see this?
    pub userhidden: bool,

    // If undiscovered, do not show.
    pub discovered: bool,

    // The title of the action.
    pub title: &'static str,
    
    // The description of the action.
    pub description: &'static str,

    // The resource cost amounts of the action.
    pub costamounts: Vec<Decimal>,

    // The resource cost types of the action.
    // pub costresources: Vec<Resource>, // NYI
    pub costresources: Vec<&'static str>,

    // The resource result amounts of the action.
    pub resultamounts: Vec<Decimal>,
}

impl Default for Action {
    fn default() -> Self {
       Action {
            available: true, 
            completed: false, 
            completable: false, 
            userhidden: false, 
            discovered: true, 
            title: "UNNAMED_ACTION", 
            description: "UNDESCRIPTED_DESCRIPTION",
            costamounts: vec![Decimal::from_number(1.0)],
            costresources: vec!["energy"],
            resultamounts: vec![Decimal::from_number(2.0)],
       }
    }
}

pub fn init_actions(cx: Scope) {
    let mut actions_iterator: Vec<Action> = Vec::new();
    actions_iterator.push(Action { available: true, title: "action1t", ..Default::default()});

    use_shared_state_provider(cx, || actions_iterator);
}

// NOTE For updating the game with new/more actions, I will need to find all Actions not in the cx and add them one at a time.
