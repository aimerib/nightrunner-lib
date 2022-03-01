use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::config::directions::Directions;
use crate::config::{Event, Item, State, Subject, VerbFunction};
use crate::parser::action::{Action, ActionType};
use crate::parser::errors::*;
use crate::util::{
    display_help, move_to_direction, parse_room_text, player_get_item, player_receive_item,
    player_remove_item, MoveSuccess,
};
use crate::NRResult;
use crate::ParsingResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
/// An enum representing the different parts of a message returned
/// by the parser when an event is successfully parsed.
pub enum MessageParts {
    /// The current text of the room. This will be the either be
    /// the current event's narrative, or the current room's narrative
    /// and the event narrative, depending on whether or not the
    /// event is marked to replace the narrative.
    RoomText,
    /// The text generated while processing the event. Primarily used
    /// to indicate when the user lost or received an item.
    EventText,
    /// A string containing all of the current room's exits and the
    /// description of the room they lead to.
    Exits,
}

/// Represents the result of parsing an event.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct EventMessage {
    /// The message to display to the user as a single string.
    pub message: String,
    /// The parts of the message to display to the user. This
    /// hashmap uses the `MessageParts` enum as the key, and
    /// the string value of the message part as the value.
    /// For more information about the variants of `MessageParts`,
    /// see the [MessageParts](MessageParts) enum.
    pub message_parts: HashMap<MessageParts, String>,
    /// Items or subjects that the front-end implementation
    /// can choose to highlight. This field can be safely ignored
    /// by the front-end if no highlighting is being implemented.
    pub templated_words: Vec<String>,
}

/// This is the function that decides what to do with the
/// input based on the action type.
pub(super) fn process_action(
    state: &Rc<RefCell<State>>,
    action: Action,
) -> NRResult<ParsingResult> {
    match action.action_type() {
        ActionType::VerbItemSubject => handle_event(&mut *state.borrow_mut(), action),
        ActionType::VerbSubject => handle_verb_subject(&mut *state.borrow_mut(), action),
        ActionType::VerbItem => handle_verb_item(state, action),
        ActionType::Verb => handle_verb(state, action),
        ActionType::Movement => handle_movement(&mut *state.borrow_mut(), action.movement),
        ActionType::Invalid => Err(InvalidEvent.into()),
    }
}

fn handle_verb(state_ref: &Rc<RefCell<State>>, action: Action) -> NRResult<ParsingResult> {
    let allowed_verbs = state_ref.borrow_mut().config.allowed_verbs.clone();
    let verb = match action.verb.clone() {
        Some(verb) => verb,
        None => return Err(InvalidVerb.into()),
    };
    if allowed_verbs.contains(&verb) {
        match &verb.verb_function {
            VerbFunction::Quit => Ok(ParsingResult::Quit),
            VerbFunction::Help => display_help(state_ref),
            VerbFunction::Look => look_room(state_ref),
            VerbFunction::Inventory => show_inventory(state_ref),
            _ => match &verb.verb_function {
                VerbFunction::Take => handle_verb_item(state_ref, action),
                VerbFunction::Drop => handle_verb_item(state_ref, action),
                VerbFunction::Talk => handle_verb_subject(&mut state_ref.borrow_mut(), action),
                VerbFunction::Normal => handle_event(&mut state_ref.borrow_mut(), action),
                _ => Err(InvalidVerb.into()),
            },
        }
    } else {
        Err(InvalidVerb.into())
    }
}

fn handle_verb_subject(state_ref: &mut State, action: Action) -> NRResult<ParsingResult> {
    let allowed_verbs = state_ref.config.allowed_verbs.clone();
    let verb = match action.verb.clone() {
        Some(verb) => verb,
        None => return Err(InvalidVerb.into()),
    };
    let subject = match action.subject.clone() {
        Some(subject) => subject,
        None => return Err(InvalidSubject.into()),
    };
    if allowed_verbs.contains(&verb) {
        if verb.verb_function == VerbFunction::Look {
            look_subject(state_ref, subject)
        } else {
            handle_event(state_ref, action)
        }
    } else {
        Err(InvalidVerb.into())
    }
}

fn handle_verb_item(state_ref: &Rc<RefCell<State>>, action: Action) -> NRResult<ParsingResult> {
    let allowed_verbs = state_ref.borrow_mut().config.allowed_verbs.clone();
    let verb = match action.verb.clone() {
        Some(verb) => verb,
        None => return Err(InvalidVerb.into()),
    };
    if allowed_verbs.contains(&verb) {
        match action.item.clone() {
            Some(item) => match &verb.verb_function {
                VerbFunction::Take => pick_item(&mut *state_ref.borrow_mut(), item),
                VerbFunction::Drop => drop_item(&mut *state_ref.borrow_mut(), item),
                VerbFunction::Look => look_item(&*state_ref.borrow(), item),
                VerbFunction::Normal => handle_event(&mut *state_ref.borrow_mut(), action),
                _ => Err(InvalidVerb.into()),
            },
            None => Err(NoItem.into()),
        }
    } else {
        Err(InvalidVerb.into())
    }
}

fn handle_movement(state_ref: &mut State, movement: Option<Directions>) -> NRResult<ParsingResult> {
    if let Some(direction) = movement {
        match move_to_direction(state_ref, direction) {
            Ok(MoveSuccess) => {
                let state_rooms = state_ref.rooms.clone();
                let current_room = match state_rooms
                    .iter()
                    .find(|room| room.id == state_ref.current_room)
                {
                    Some(room) => room,
                    None => return Err(InvalidRoom.into()),
                };
                let narrative = match state_ref
                    .config
                    .narratives
                    .iter()
                    .find(|n| n.id == current_room.narrative)
                {
                    Some(narrative) => narrative,
                    None => return Err(InvalidNarrative.into()),
                };
                let new_room_text = parse_room_text(
                    state_ref.clone(),
                    narrative.text.clone(),
                    "".to_string(),
                    None,
                )?;
                Ok(ParsingResult::EventSuccess(new_room_text))
            }
            Err(error) => Err(error),
        }
    } else {
        Err(InvalidDirection.into())
    }
}

fn handle_event(state: &mut State, action: Action) -> NRResult<ParsingResult> {
    let mut event_messages_vec: Vec<String> = Vec::new();

    let current_room_id = state.current_room;
    let state_rooms = state.rooms.clone();
    let current_room = match state_rooms.iter().find(|room| room.id == current_room_id) {
        Some(room) => room,
        None => return Err(InvalidRoom.into()),
    };
    // let state_narratives = state.config.narratives.clone();
    let state_events = state.config.events.clone();
    let state_items = state.config.items.clone();

    let (inventory_item, subject) = extract_item_subject(state, &action);
    let mut events: Vec<&Event> = vec![];

    if let (Some(verb), Some(subject), Some(inventory_item)) =
        (action.verb.clone(), subject.clone(), inventory_item.clone())
    {
        events = current_room
            .events
            .iter()
            .filter(|event| {
                // let event = match state_events
                //     .iter()
                //     .find(|state_event| state_event.id == room_event.id)
                // {
                //     Some(event) => event,
                //     None => return false,
                // };
                let required_verb_id = match event.required_verb {
                    Some(verb) => verb,
                    None => return false,
                };
                let required_subject_id = match event.required_subject {
                    Some(subject) => subject,
                    None => return false,
                };
                let required_item_id = match event.required_item {
                    Some(item) => item,
                    None => return false,
                };
                required_item_id == inventory_item.id
                    && required_verb_id == verb.id
                    && required_subject_id == subject.id
            })
            .collect();
    }
    // if action verb and subject exists, process action.
    // every event requires at least one verb and one subject
    // some events might also require an item.
    else if let (Some(verb), Some(subject), None) =
        (action.verb.clone(), subject.clone(), inventory_item.clone())
    {
        // each room has a list of events. the event_id is derived from the list
        // of events in a room. If the action verb matches the required verb for the event
        // and the action subject matches the event subject, we return an Option with the id
        // of the event or None
        events = current_room
            .events
            .iter()
            .filter(|event| {
                // let event = match state_events
                //     .iter()
                //     .find(|state_event| state_event.id == room_event.id)
                // {
                //     Some(event) => event,
                //     None => return false,
                // };
                let required_verb_id = match event.required_verb {
                    Some(verb) => verb,
                    None => return false,
                };
                let required_subject_id = match event.required_subject {
                    Some(subject) => subject,
                    None => return false,
                };
                required_verb_id == verb.id
                    && required_subject_id == subject.id
                    && event.required_item.is_none()
            })
            .collect();
    } else if let (Some(verb), None, Some(inventory_item)) =
        (action.verb, subject.clone(), inventory_item)
    {
        // each room has a list of events. the event_id is derived from the list
        // of events in a room. If the action verb matches the required verb for the event
        // and the action subject matches the event subject, we return an Option with the id
        // of the event or None
        events = current_room
            .events
            .iter()
            .filter(|event| {
                // let event = match state_events
                //     .iter()
                //     .find(|state_event| state_event.id == room_event.id)
                // {
                //     Some(event) => event,
                //     None => return false,
                // };
                let required_verb_id = match event.required_verb {
                    Some(verb) => verb,
                    None => return false,
                };
                let required_item_id = match event.required_item {
                    Some(item_id) => item_id,
                    None => return false,
                };
                required_verb_id == verb.id
                    && required_item_id == inventory_item.id
                    && event.required_subject.is_none()
            })
            .collect();
    }

    // We only want to continue from this point on if we have a valid event_id
    // However, if the player tries to perform an action on a subject that isn't
    // associated with an event at the moment, we want to return that subject's
    // default text.
    let event = if !events.is_empty() {
        match events.iter().find(|room_event| {
            match state_events
                .iter()
                .find(|state_event| state_event.id == room_event.id)
            {
                Some(event) => !event.completed,
                None => false,
            }
        }) {
            Some(event) => event,
            None => {
                if let Some(subject) = subject {
                    return Ok(ParsingResult::SubjectNoEvent(subject.default_text));
                } else {
                    return Err(InvalidEvent.into());
                }
            }
        }
    } else if let Some(subject) = subject {
        return Ok(ParsingResult::SubjectNoEvent(subject.default_text));
    } else {
        return Err(InvalidEvent.into());
    };

    // let event = match state_events.iter().find(|event| event.id == **event_id) {
    //     Some(event) => event,
    //     None => return Err(InvalidEvent.into()),
    // };

    let required_events_completed = state
        .config
        .events
        .clone()
        .iter()
        .filter(|e| event.required_events.clone().contains(&e.id))
        .all(|e| e.completed);
    if !required_events_completed {
        return Err(RequiredEventNotCompleted.into());
    }

    if let Some(item_id) = &event.add_item {
        // add the item to the user inventory
        if let Some(item) = state_items.iter().find(|i| i.id == *item_id) {
            event_messages_vec.push(player_receive_item(state, item.clone()));
        }
    }
    // if the event removes an item from the user inventory, retrieve the item_id
    if let Some(item_id) = match state_events
        .iter()
        .find(|state_event| state_event.id == event.id)
    {
        Some(event) => event.remove_item,
        None => return Err(InvalidEvent.into()),
    } {
        // remove the item to the user inventory
        if let Some(item) = state_items.iter().find(|i| i.id == item_id) {
            match player_remove_item(&mut state.player, item.clone()) {
                Ok(message) => event_messages_vec.push(message),
                Err(message) => event_messages_vec.push(
                    message
                        .downcast::<Box<super::errors::NoItem>>()
                        .unwrap()
                        .to_string(),
                ),
            };
        }
    }

    // Handle moving subjects around if event requires that
    if event.remove_subject {
        let event_subject = match subject {
            Some(subject) => subject,
            None => return Err(InvalidEvent.into()),
        };
        if let Some(location) = event.move_subject_to_location {
            state
                .rooms
                .iter_mut()
                .find(|room| room.id == current_room_id)
                .unwrap()
                .remove_subject(event_subject.id);
            state
                .rooms
                .iter_mut()
                .find(|r| r.id == location)
                .unwrap()
                .add_subject(event_subject);
        } else {
            state
                .rooms
                .iter_mut()
                .find(|room| room.id == current_room_id)
                .unwrap()
                .remove_subject(event_subject.id);
        }
    }
    if let Some(new_subject_id) = event.add_subject {
        let new_subject = state
            .config
            .subjects
            .iter()
            .find(|s| s.id == new_subject_id)
            .unwrap();
        state
            .rooms
            .iter_mut()
            .find(|room| room.id == current_room_id)
            .unwrap()
            .add_subject(new_subject.clone());
    }
    // if let Some(subject) = subject {
    //     match player_remove_subject(&mut state.player, subject.clone()) {
    //         Ok(message) => event_messages_vec.push(message),
    //         Err(message) => event_messages_vec.push(
    //             message
    //                 .downcast::<Box<super::errors::NoSubject>>()
    //                 .unwrap()
    //                 .to_string(),
    //         ),
    //     };
    // }
    if event.remove_old_narrative {
        if let Some(narrative_after) = event.narrative_after {
            state.set_narrative(narrative_after);
        }
        // state
        //     .rooms
        //     .iter_mut()
        //     .find(|room| room.id == current_room_id)
        //     .unwrap()
        //     .narrative = event.narrative.unwrap();
    }

    let event_message = event_messages_vec
        .clone()
        .iter()
        .filter(|s| !&(*s).clone().is_empty())
        .cloned()
        .collect::<Vec<String>>()
        .join("");

    // if the event replaces current narrative, we set the current narrative to the event narrative
    // and add the current narrative to the event messages
    let event_message = return_formated_message(event, state, event_message)?;
    // completes the event so it can't be repeated
    if let Some(event) = state
        .config
        .events
        .iter_mut()
        .find(|state_event| state_event.id == event.id)
    {
        if event.destination.is_some() {
            state.current_room = event.destination.unwrap();
        }
        event.completed = true;
    }
    Ok(event_message)
}

fn return_formated_message(
    event: &crate::config::Event,
    state: &mut State,
    event_message: String,
    // state_narratives: &[Narrative],
) -> NRResult<ParsingResult> {
    let state_ref = state.clone();
    // if event.remove_old_narrative {
    match state
        .config
        .narratives
        .iter()
        .find(|narrative| match event.narrative {
            Some(narrative_id) => narrative.id == narrative_id,
            None => false,
        }) {
        Some(narrative) => {
            let new_room_text = parse_room_text(
                state_ref,
                narrative.text.clone(),
                event_message,
                Some(event.id),
            )?;
            Ok(ParsingResult::EventSuccess(new_room_text))
        }
        None => Err(InvalidNarrative.into()),
    }
    // } else {
    //     match state_narratives
    //         .iter()
    //         .find(|narrative| match event.narrative {
    //             Some(narrative_id) => narrative.id == narrative_id,
    //             None => false,
    //         }) {
    //         Some(narrative) => {
    //             let room_narrative_id = state
    //                 .rooms
    //                 .iter()
    //                 .find(|room| room.id == state.current_room)
    //                 .unwrap()
    //                 .narrative;
    //             let room_narrative = state_narratives
    //                 .iter()
    //                 .find(|r_narrative| r_narrative.id == room_narrative_id)
    //                 .unwrap();
    //             let room_text =
    //                 room_narrative.text.clone() + "\n\n" + narrative.text.clone().as_str();
    //             let new_room_text =
    //                 parse_room_text(state_ref, room_text, event_message, Some(event.id))?;

    //             Ok(ParsingResult::EventSuccess(new_room_text))
    //         }
    //         None => Err(InvalidNarrative.into()),
    //     }
    // }
}

fn extract_item_subject(state: &State, action: &Action) -> (Option<Item>, Option<Subject>) {
    let current_room_id = state.current_room;
    let state_rooms = state.rooms.clone();
    let current_room = match state_rooms.iter().find(|room| room.id == current_room_id) {
        Some(room) => room,
        None => return (None, None),
    };
    let inventory_item = match action.item.clone() {
        None => None,
        Some(item) => {
            if state
                .player
                .inventory
                .items
                .iter()
                .any(|player_item| player_item.id == item.id)
            {
                Some(item)
            } else {
                None
            }
        }
    };
    let subject = match action.subject.clone() {
        None => None,
        Some(action_subject) => {
            if current_room
                .subjects
                .iter()
                .any(|s| s.id == action_subject.id)
            {
                Some(action_subject)
            } else {
                None
            }
        }
    };
    (inventory_item, subject)
}

fn show_inventory(state: &RefCell<State>) -> NRResult<ParsingResult> {
    let player_items = state.borrow_mut().player.inventory.items.clone();
    let items: Vec<String> = player_items
        .iter()
        .map(|item| {
            let mut item_name = item.name.clone();
            let first_char = &item_name.to_lowercase().chars().next().unwrap();
            if ['a', 'e', 'i', 'o', 'u'].contains(first_char) {
                item_name.insert_str(0, "an ");
            } else {
                item_name.insert_str(0, "a ");
            }
            item_name
        })
        .collect();
    if !items.is_empty() {
        let mut items_string = items.join("\n");
        items_string.insert_str(0, "You are currently carrying: \n\n");
        Ok(ParsingResult::Inventory(items_string))
    } else {
        Ok(ParsingResult::Inventory(
            "You are not carrying anything.".to_string(),
        ))
    }
}

fn pick_item(state: &mut State, item: Item) -> NRResult<ParsingResult> {
    let current_room_id = state.current_room;
    let current_room = match state.rooms.iter().find(|room| room.id == current_room_id) {
        Some(room) => room,
        None => return Err(NoRoom.into()),
    };
    let room_items = &current_room.stash.items;
    if room_items.contains(&item) {
        if item.can_pick {
            player_get_item(state, item)
        } else {
            Err(CantPick.into())
        }
    } else {
        Err(NoItem.into())
    }
}

fn drop_item(state: &mut State, item: Item) -> NRResult<ParsingResult> {
    let current_room_id = state.current_room;
    let current_room = match state
        .rooms
        .iter_mut()
        .find(|room| room.id == current_room_id)
    {
        Some(room) => room,
        None => return Err(NoRoom.into()),
    };
    let player = &mut state.player;
    let room_items = &mut current_room.stash;
    if player.inventory.items.contains(&item) {
        match player_remove_item(player, item.clone()) {
            Ok(message) => {
                room_items.add_item(item);
                Ok(ParsingResult::DropItem(message))
            }
            Err(_) => Err(NoItem.into()),
        }
    } else {
        Err(NoItem.into())
    }
}

fn look_item(state: &State, item: Item) -> NRResult<ParsingResult> {
    let current_room_id = state.current_room;
    let inventory = &state.player.inventory;
    let current_room = match state.rooms.iter().find(|room| room.id == current_room_id) {
        Some(room) => room,
        None => return Err(NoRoom.into()),
    };
    let room_items = &current_room.stash.items;
    let inventory_items = &inventory.items;

    if room_items.contains(&item) || inventory_items.contains(&item) {
        Ok(ParsingResult::Look(item.description))
    } else {
        Ok(ParsingResult::Look("I can't see that here".to_string()))
    }
}

fn look_subject(state: &State, subject: Subject) -> NRResult<ParsingResult> {
    let current_room_id = state.current_room;
    let current_room = match state.rooms.iter().find(|room| room.id == current_room_id) {
        Some(room) => room,
        None => return Err(NoRoom {}.into()),
    };
    let room_subjects = &current_room.subjects;

    if room_subjects.contains(&subject) {
        Ok(ParsingResult::Look(subject.description))
    } else {
        Ok(ParsingResult::Look("I can't see that here".to_string()))
    }
}

fn look_room(state: &RefCell<State>) -> NRResult<ParsingResult> {
    let current_room_id = state.borrow().current_room;
    let rooms = state.borrow().rooms.clone();
    let current_room = match rooms.iter().find(|room| room.id == current_room_id) {
        Some(room) => room,
        None => return Err(NoRoom.into()),
    };
    let room_subjects = current_room
        .subjects
        .clone()
        .iter()
        .map(|subject| subject.name.clone())
        .collect::<Vec<String>>()
        .join("\n");
    let description = &current_room.description;
    let items = current_room.stash.items.clone();

    let items_descriptions = if !items.is_empty() {
        format!(
            "Here you see: \n{}",
            items
                .iter()
                .clone()
                .map(|item| {
                    let first_char = &item.name.to_lowercase().chars().next().unwrap();
                    if ['a', 'e', 'i', 'o', 'u'].contains(first_char) {
                        format!("an {}", item.name)
                    } else {
                        format!("a {}", &item)
                    }
                })
                .collect::<Vec<String>>()
                .join("\n")
        )
    } else {
        "".to_string()
    };
    let mut room_description = if items_descriptions.is_empty() {
        description.to_string()
    } else {
        format!("{}\n\n{}", description, items_descriptions)
    };
    if !room_subjects.is_empty() {
        room_description.push_str(&format!("\n{}", room_subjects));
    }

    Ok(ParsingResult::Look(room_description))
}

#[cfg(test)]
#[path = "interpreter_tests.rs"]
mod interpreter_tests;
