#![allow(non_snake_case)]

use std::collections::HashMap;

/// This is all the information for Actions.

use dioxus::prelude::*;
use break_eternity::Decimal;

use crate::resource::Resource;

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
    pub costresources: Vec<Resource>, // NYI
    // pub costresources: Vec<&'static str>,

    // The resource result amounts of the action.
    pub resultamounts: Vec<Decimal>,

    // The resource result types of the action.
    pub resultresources: Vec<Resource>, // NYI
    // pub resultresources: Vec<&'static str>,
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
            description: "UNDESCRIPTED_ACTION_DESCRIPTION",
            costamounts: vec![Decimal::from_number(1.0)],
            costresources: vec![Resource { ..Default::default() }],
            resultamounts: vec![Decimal::from_number(2.0)],
            resultresources: vec![Resource { ..Default::default() }],
       }
    }
}

// Put all actions into one iterator for later use.
pub fn init_actions(cx: Scope) {
    let resources = use_shared_state::<HashMap<&'static str, Resource>>(cx).unwrap().read();


    let mut actions_iterator: Vec<Action> = Vec::new();
    actions_iterator.push(Action { available: true, title: "action1t", costresources: vec![resources["destiny"]], ..Default::default()});

    use_shared_state_provider(cx, || actions_iterator);
}

// NOTE For updating the game with new/more actions, I will need to find all Actions not in the cx and add them one at a time.
