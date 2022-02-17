use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::config::VerbFunction;
use crate::config::{directions::Directions, Subject};
use crate::config::{rooms::Item, State};
use crate::parser::action::{Action, ActionType};
use crate::parser::errors::*;
use crate::util::{
    display_help, move_to_direction, parse_room_text, player_get_item, player_receive_item,
    player_remove_item, MoveSuccess,
};
use crate::NRResult;
use serde::{Deserialize, Serialize};

/// This is the result of the parsing of the input.
/// Each variant contains the output for the game and
/// should be used by a front-end to display to the user.
///
/// ```rust, ignore
/// pub enum ParsingResult {
///     Movement(MoveSuccess),
///     Help(String),
///     Look(String),
///     NewItem(String),
///     DropItem(String),
///     Inventory(String),
///     SubjectNoEvent(String),
///     EventSuccess(EventMessage),
///     Quit,
/// }
/// ```
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]

pub enum ParsingResult {
    Help(String),
    Look(String),
    NewItem(String),
    DropItem(String),
    Inventory(String),
    SubjectNoEvent(String),
    EventSuccess(EventMessage),
    Quit,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum MessageParts {
    RoomText,
    EventText,
    Exits,
}

/// Represents the result of parsing an event. This struct
/// contains only two fields:
/// * `message` - The message to display to the user.
/// * `templated_words` - Items or subjects that the
/// front-end implementation can choose to highlight. This
/// field can be safely ignored by the front-end.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct EventMessage {
    pub message: String,
    pub message_parts: HashMap<MessageParts, String>,
    pub templated_words: Vec<String>,
}

/// This is the function that decides what to do with the
/// input based on the action type.
pub fn process_action(state: &Rc<RefCell<State>>, action: Action) -> NRResult<ParsingResult> {
    match action.action_type() {
        ActionType::VerbItemSubject => handle_event(&mut *state.borrow_mut(), action),
        ActionType::VerbSubject => handle_verb_subject(&mut *state.borrow_mut(), action),
        ActionType::VerbItem => handle_verb_item(&state, action.clone()),
        ActionType::Verb => handle_verb(&state, action.clone()),
        ActionType::Movement => handle_movement(&mut *state.borrow_mut(), action.movement.clone()),
        ActionType::Invalid => Err(InvalidAction.into()),
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
                VerbFunction::Take => handle_verb_item(state_ref, action.clone()),
                VerbFunction::Drop => handle_verb_item(state_ref, action.clone()),
                VerbFunction::Talk => {
                    handle_verb_subject(&mut state_ref.borrow_mut(), action.clone())
                }
                VerbFunction::Normal => handle_event(&mut state_ref.borrow_mut(), action.clone()),
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
            look_subject(&state_ref, subject)
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

    let current_room_id = state.current_room.clone();
    let state_rooms = state.rooms.clone();
    let current_room = match state_rooms.iter().find(|room| room.id == current_room_id) {
        Some(room) => room,
        None => return Err(InvalidRoom.into()),
    };
    let state_narratives = state.config.narratives.clone();
    let state_events = state.config.events.clone();
    let state_items = state.config.items.clone();

    let (inventory_item, subject) = extract_item_subject(&state, &action);
    let mut event_ids: Vec<&u16> = vec![];

    if let (Some(verb), Some(subject), Some(inventory_item)) =
        (action.verb.clone(), subject.clone(), inventory_item.clone())
    {
        event_ids = current_room
            .room_events
            .iter()
            .filter(|event_id| {
                let event = match state_events.iter().find(|event| event.id == **event_id) {
                    Some(event) => event,
                    None => return false,
                };
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
            .collect::<Vec<&u16>>();
    }
    // if action verb and subject exists, process action.
    // every event requires at least one verb and one subject
    // some events might also require an item.
    else if let (Some(verb), Some(subject), None) =
        (action.verb.clone(), subject.clone(), inventory_item)
    {
        // each room has a list of events. the event_id is derived from the list
        // of events in a room. If the action verb matches the required verb for the event
        // and the action subject matches the event subject, we return an Option with the id
        // of the event or None
        event_ids = current_room
            .room_events
            .iter()
            .filter(|event_id| {
                let event = match state_events.iter().find(|event| event.id == **event_id) {
                    Some(event) => event,
                    None => return false,
                };
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
            .collect::<Vec<&u16>>();
    }

    // We only want to continue from this point on if we have a valid event_id
    // However, if the player tries to perform an action on a subject that isn't
    // associated with an event at the moment, we want to return that subject's
    // default text.
    let event_id = if &event_ids.len() > &0 {
        match event_ids.iter().find(|e_id| {
            match state_events.iter().find(|event| event.id == ***e_id) {
                Some(event) => event.completed == false,
                None => false,
            }
        }) {
            Some(event_id) => event_id,
            None => {
                if subject.is_some() {
                    return Ok(ParsingResult::SubjectNoEvent(
                        subject.unwrap().default_text.clone(),
                    ));
                } else {
                    return Err(InvalidEvent.into());
                }
            }
        }
    } else {
        return Err(InvalidEvent.into());
    };

    let event = match state_events.iter().find(|event| event.id == **event_id) {
        Some(event) => event,
        None => return Err(InvalidEvent.into()),
    };

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
        if let Some(item) = state_items.iter().find(|i| &i.id == &item_id.clone()) {
            event_messages_vec.push(player_receive_item(state, item.clone()));
        }
    }
    // if the event removes an item from the user inventory, retrieve the item_id
    if let Some(item_id) = match state_events.iter().find(|event| event.id == **event_id) {
        Some(event) => event.remove_item.clone(),
        None => return Err(InvalidEvent.into()),
    } {
        // remove the item to the user inventory
        if let Some(item) = state_items.iter().find(|i| &i.id == &item_id.clone()) {
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
    let event_message = event_messages_vec
        .clone()
        .iter()
        .filter(|s| s.clone() != "")
        .map(|s| s.clone())
        .collect::<Vec<String>>()
        .join("");

    // if the event replaces current narrative, we set the current narrative to the event narrative
    // and add the current narrative to the event messages
    let event_message = return_formated_message(event, state, event_message, &state_narratives)?;
    // completes the event so it can't be repeated
    if let Some(event) = state.config.events.iter_mut().find(|e| &**e == event) {
        if event.destination.is_some() {
            state.current_room = event.destination.clone().unwrap();
        }
        event.completed = true;
    }
    Ok(event_message)
}

fn return_formated_message(
    event: &crate::config::Event,
    state: &mut State,
    event_message: String,
    state_narratives: &Vec<crate::config::Narrative>,
) -> NRResult<ParsingResult> {
    let state_ref = state.clone();
    if event.remove_old_narrative {
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
                    state_ref.clone(),
                    narrative.text.clone(),
                    event_message,
                    Some(event.id),
                )?;
                return Ok(ParsingResult::EventSuccess(new_room_text));
            }
            None => return Err(InvalidNarrative.into()),
        }
    } else {
        match state_narratives
            .iter()
            .find(|narrative| match event.narrative {
                Some(narrative_id) => narrative.id == narrative_id,
                None => false,
            }) {
            Some(narrative) => {
                let room_narrative_id = state
                    .rooms
                    .iter()
                    .find(|room| room.id == state.current_room)
                    .unwrap()
                    .narrative;
                let room_narrative = state_narratives
                    .iter()
                    .find(|r_narrative| r_narrative.id == room_narrative_id)
                    .unwrap();
                let room_text =
                    room_narrative.text.clone() + "\n\n" + narrative.text.clone().as_str();
                let new_room_text =
                    parse_room_text(state_ref.clone(), room_text, event_message, Some(event.id))?;

                return Ok(ParsingResult::EventSuccess(new_room_text));
            }
            None => return Err(InvalidNarrative.into()),
        }
    }
}

fn extract_item_subject(state: &State, action: &Action) -> (Option<Item>, Option<Subject>) {
    let current_room_id = state.current_room.clone();
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
        Some(state_subject) => {
            if current_room.subjects.contains(&state_subject.id) {
                Some(state_subject)
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
            item_name.insert_str(0, "a ");
            item_name
        })
        .collect();
    if items.len() > 0 {
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
            player_get_item(state, item.clone())
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
                return Ok(ParsingResult::DropItem(message));
            }
            Err(_) => return Err(NoItem.into()),
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

    if room_items.contains(&item) {
        Ok(ParsingResult::Look(item.description.clone()))
    } else if inventory.items.contains(&item) {
        Ok(ParsingResult::Look(item.description.clone()))
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

    if room_subjects.contains(&subject.id) {
        Ok(ParsingResult::Look(subject.description))
    } else {
        Ok(ParsingResult::Look("I can't see that here".to_string()))
    }
}

fn look_room(state: &RefCell<State>) -> NRResult<ParsingResult> {
    let current_room_id = state.borrow().current_room.clone();
    let rooms = state.borrow().rooms.clone();
    let current_room = match rooms.iter().find(|room| room.id == current_room_id).clone() {
        Some(room) => room,
        None => return Err(NoRoom.into()),
    };
    let description = &current_room.description;
    let items = current_room.stash.items.clone();

    let items_descriptions = if items.len() > 0 {
        format!(
            "Here you see: \n\n{}",
            items
                .iter()
                .clone()
                .map(|item| { format!("a {}", &item) })
                .collect::<Vec<String>>()
                .join("\n")
        )
    } else {
        "".to_string()
    };
    let room_description = if items_descriptions.is_empty() {
        format!("{}", description)
    } else {
        format!("{}\n{}", description, items_descriptions)
    };

    Ok(ParsingResult::Look(room_description))
}

#[cfg(test)]
#[path = "interpreter_tests.rs"]
mod interpreter_tests;
