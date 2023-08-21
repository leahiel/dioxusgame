/// Actions are clickable events that manipulate Resources when used.
/// Not to be confused with: Story_Actions.

use dioxus::prelude::*;
use std::collections::HashMap;
use break_eternity::Decimal;

use crate::resource::Resource;

#[derive(Clone)]
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
    pub costresources: Vec<Resource>,

    // The resource result amounts of the action.
    pub resultamounts: Vec<Decimal>,

    // The resource result types of the action.
    pub resultresources: Vec<Resource>,
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
            costresources: vec![Resource { ..Default::default() }],
            costamounts: vec![Decimal::from_number(1.0)],
            resultresources: vec![Resource { ..Default::default() }],
            resultamounts: vec![Decimal::from_number(2.0)],
       }
    }
}

/// Creates a HashMap [key: &'static str, value: Action]
/// 
/// This HashMap contains every Action, which is then added to the shared state.
/// 
/// Access with:
/// ```
/// let mut varname = use_shared_state::<HashMap<&'static str, Action>>(cx).unwrap().read();
/// ```
pub fn init_actions(cx: Scope) {
    // Load Resources for use in Actions
    let resources = use_shared_state::<HashMap<&'static str, Resource>>(cx).unwrap().read();

    // Create Action HashMap
    let mut actions_hashmap: HashMap<&'static str, Action> = HashMap::new();

    // Add Actions to HashMap
    actions_hashmap.insert("action1", Action { available: true, title: "action1t", costresources: vec![resources["destiny"]], ..Default::default()});

    // Give HashMap to SharedState.
    use_shared_state_provider(cx, || actions_hashmap);
}

// NOTE For updating the game with new/more actions, I will need to find all Actions not in the cx and add them one at a time.
