use std::collections::HashMap;

use crate::config::directions::Directions;
use crate::config::types::Room;
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
    /// see the [MessageParts] enum.
    pub message_parts: HashMap<MessageParts, String>,
    /// Items or subjects that the front-end implementation
    /// can choose to highlight. This field can be safely ignored
    /// by the front-end if no highlighting is being implemented.
    pub templated_words: Vec<String>,
}

/// This is the function that decides what to do with the
/// input based on the action type.
pub(super) fn process_action(state: &State, action: Action) -> NRResult<(State, ParsingResult)> {
    match action.action_type() {
        ActionType::VerbItemSubject => handle_event(state, action),
        ActionType::VerbSubject => handle_verb_subject(state, action),
        ActionType::VerbItem => handle_verb_item(state, action),
        ActionType::Verb => handle_verb(state, action),
        ActionType::Movement => handle_movement(state, action.movement),
        ActionType::Invalid => Err(InvalidEvent.into()),
    }
}

fn handle_verb(state: &State, action: Action) -> NRResult<(State, ParsingResult)> {
    let new_state = state.clone();
    let allowed_verbs = state.config.allowed_verbs.clone();
    let verb = match action.verb.clone() {
        Some(verb) => verb,
        None => return Err(InvalidVerb.into()),
    };
    if allowed_verbs.contains(&verb) {
        match &verb.verb_function {
            VerbFunction::Quit => Ok((new_state, ParsingResult::Quit)),
            VerbFunction::Help => match display_help(&new_state) {
                Ok(help_text) => Ok((new_state, help_text)),
                Err(error) => Err(error),
            },
            VerbFunction::Look => match look_room(&new_state) {
                Ok(parsing_result) => Ok((new_state, parsing_result)),
                Err(error) => Err(error),
            },
            VerbFunction::Inventory => match show_inventory(&new_state) {
                Ok(parsing_result) => Ok((new_state, parsing_result)),
                Err(error) => Err(error),
            },
            _ => match &verb.verb_function {
                VerbFunction::Take => handle_verb_item(state, action),
                VerbFunction::Drop => handle_verb_item(state, action),
                VerbFunction::Talk => handle_verb_subject(state, action),
                VerbFunction::Normal => handle_event(state, action),
                _ => Err(InvalidVerb.into()),
            },
        }
    } else {
        Err(InvalidVerb.into())
    }
}

fn handle_verb_subject(state: &State, action: Action) -> NRResult<(State, ParsingResult)> {
    let allowed_verbs = state.config.allowed_verbs.clone();
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
            match look_subject(state, subject) {
                Ok(parsing_result) => Ok((state.clone(), parsing_result)),
                Err(error) => Err(error),
            }
        } else {
            handle_event(state, action)
        }
    } else {
        Err(InvalidVerb.into())
    }
}

fn handle_verb_item(state: &State, action: Action) -> NRResult<(State, ParsingResult)> {
    let allowed_verbs = state.config.allowed_verbs.clone();
    let verb = match action.verb.clone() {
        Some(verb) => verb,
        None => return Err(InvalidVerb.into()),
    };
    if allowed_verbs.contains(&verb) {
        match action.item.clone() {
            Some(item) => match &verb.verb_function {
                VerbFunction::Take => pick_item(state, item),
                VerbFunction::Drop => drop_item(state, item),
                VerbFunction::Look => match look_item(state, item) {
                    Ok(parsing_result) => Ok((state.clone(), parsing_result)),
                    Err(error) => Err(error),
                },
                VerbFunction::Normal => handle_event(state, action),
                _ => Err(InvalidVerb.into()),
            },
            None => Err(NoItem.into()),
        }
    } else {
        Err(InvalidVerb.into())
    }
}

fn handle_movement(
    state: &State,
    movement: Option<Directions>,
) -> NRResult<(State, ParsingResult)> {
    if let Some(direction) = movement {
        match move_to_direction(state, direction) {
            Ok((new_state, MoveSuccess)) => {
                let state_rooms = new_state.rooms.clone();
                let current_room = match state_rooms
                    .iter()
                    .find(|room| room.id == new_state.current_room)
                {
                    Some(room) => room,
                    None => return Err(InvalidRoom.into()),
                };
                let narrative = match new_state
                    .config
                    .narratives
                    .iter()
                    .find(|n| n.id == current_room.narrative)
                {
                    Some(narrative) => narrative,
                    None => return Err(InvalidNarrative.into()),
                };
                let new_room_text =
                    parse_room_text(&new_state, narrative.text.clone(), "".to_string(), None)?;
                Ok((new_state, ParsingResult::EventSuccess(new_room_text)))
            }
            Err(error) => Err(error),
        }
    } else {
        Err(InvalidDirection.into())
    }
}

fn handle_event(state: &State, action: Action) -> NRResult<(State, ParsingResult)> {
    let current_room_id = state.current_room;
    let current_room = state
        .rooms
        .iter()
        .find(|room| room.id == current_room_id)
        .ok_or(InvalidRoom)?;

    let (inventory_item, subject) = extract_item_subject(state, &action);
    let events = filter_events(current_room, &action, &inventory_item, &subject);

    if events.is_empty() {
        if let Some(subject) = subject {
            return Ok((
                state.clone(),
                ParsingResult::SubjectNoEvent(subject.default_text.clone()),
            ));
        } else {
            return Err(InvalidEvent.into());
        }
    }

    let event = events
        .iter()
        .find(|event| !event.is_completed())
        .ok_or(InvalidEvent)?;

    if !are_required_events_completed(state, event)? {
        return Err(RequiredEventNotCompleted.into());
    }

    let mut new_state = state.clone();
    let (state, event_messages) = process_event(&new_state, event, &subject)?;
    new_state = state;

    let event_message = format_event_message(event, &new_state, &event_messages)?;
    new_state.complete_event(event.id);

    if let Some(destination) = event.destination {
        new_state.current_room = destination;
    }

    Ok((new_state, event_message))
}

fn filter_events<'a>(
    room: &'a Room,
    action: &Action,
    inventory_item: &Option<Item>,
    subject: &Option<Subject>,
) -> Vec<&'a Event> {
    room.events
        .iter()
        .filter(
            |event| match (action.verb.as_ref(), subject, inventory_item) {
                (Some(verb), Some(subject), Some(item)) => {
                    event.required_verb == Some(verb.id)
                        && event.required_subject == Some(subject.id)
                        && event.required_item == Some(item.id)
                }
                (Some(verb), Some(subject), None) => {
                    event.required_verb == Some(verb.id)
                        && event.required_subject == Some(subject.id)
                        && event.required_item.is_none()
                }
                (Some(verb), None, Some(item)) => {
                    event.required_verb == Some(verb.id)
                        && event.required_item == Some(item.id)
                        && event.required_subject.is_none()
                }
                _ => false,
            },
        )
        .collect()
}

fn are_required_events_completed(state: &State, event: &Event) -> NRResult<bool> {
    let required_events_completed = event
        .required_events
        .iter()
        .all(|event_id| state.is_event_completed(*event_id));

    Ok(required_events_completed)
}

fn process_event(
    state: &State,
    event: &Event,
    subject: &Option<Subject>,
) -> NRResult<(State, Vec<String>)> {
    let mut new_state = state.clone();
    let mut event_messages = Vec::new();

    if let Some(item_id) = event.add_item {
        if let Some(item) = new_state.config.items.iter().find(|i| i.id == item_id) {
            let (state, message) = player_receive_item(&new_state, item.clone())?;
            new_state = state;
            event_messages.push(message);
        }
    }

    if let Some(item_id) = event.remove_item {
        if let Some(item) = new_state.config.items.iter().find(|i| i.id == item_id) {
            let (state, message) = player_remove_item(&new_state, item.clone())?;
            new_state = state;
            event_messages.push(message);
        }
    }

    let (state, _) = process_subject_movement(&new_state, event, subject)?;
    new_state = state;
    let (state, _) = process_subject_addition(&new_state, event)?;
    new_state = state;

    if event.remove_old_narrative {
        if let Some(narrative_after) = event.narrative_after {
            new_state.set_narrative(narrative_after);
        }
    }

    Ok((new_state, event_messages))
}

fn process_subject_movement(
    state: &State,
    event: &Event,
    subject: &Option<Subject>,
) -> NRResult<(State, ())> {
    let mut new_state = state.clone();
    if event.remove_subject {
        let event_subject = subject.as_ref().ok_or(InvalidEvent)?;
        if let Some(location) = event.move_subject_to_location {
            new_state.move_subject(event_subject.id, location)?;
        } else {
            new_state.remove_subject(event_subject.id)?;
        }
    }
    Ok((new_state, ()))
}

fn process_subject_addition(state: &State, event: &Event) -> NRResult<(State, ())> {
    let mut new_state = state.clone();
    if let Some(new_subject_id) = event.add_subject {
        let new_subject = new_state
            .config
            .subjects
            .iter()
            .find(|s| s.id == new_subject_id)
            .ok_or(InvalidEvent)?;
        new_state.add_subject(new_subject.clone())?;
    }
    Ok((new_state, ()))
}

fn format_event_message(
    event: &Event,
    state: &State,
    event_messages: &[String],
) -> NRResult<ParsingResult> {
    let event_message = event_messages
        .iter()
        .filter(|s| !s.is_empty())
        .cloned()
        .collect::<Vec<String>>()
        .join("");

    return_formated_message(event, state, event_message)
}

fn return_formated_message(
    event: &crate::config::Event,
    state: &State,
    event_message: String,
) -> NRResult<ParsingResult> {
    let event_narrative = state
        .config
        .narratives
        .iter()
        .find(|narrative| event.narrative.map_or(false, |id| narrative.id == id))
        .ok_or(InvalidNarrative)?;

    let room_text = if event.remove_old_narrative {
        event_narrative.text.clone()
    } else {
        let room_narrative_id = state
            .rooms
            .iter()
            .find(|room| room.id == state.current_room)
            .unwrap()
            .narrative;
        let room_narrative = state
            .config
            .narratives
            .iter()
            .find(|r_narrative| r_narrative.id == room_narrative_id)
            .unwrap();
        room_narrative.text.clone() + "\n\n" + event_narrative.text.as_str()
    };

    let new_room_text = parse_room_text(state, room_text, event_message, Some(event.id))?;
    Ok(ParsingResult::EventSuccess(new_room_text))
}

fn extract_item_subject(state: &State, action: &Action) -> (Option<Item>, Option<Subject>) {
    let current_room_id = state.current_room;
    let state_rooms = state.rooms.clone();
    let current_room = match state_rooms.iter().find(|room| room.id == current_room_id) {
        Some(room) => room,
        None => return (None, None),
    };
    let inventory_item = action.item.clone().filter(|item| {
        state
            .player
            .inventory
            .items
            .iter()
            .any(|player_item| player_item.id == item.id)
    });
    let subject = action.subject.clone().filter(|action_subject| {
        current_room
            .subjects
            .iter()
            .any(|s| s.id == action_subject.id)
    });
    (inventory_item, subject)
}

fn show_inventory(state: &State) -> NRResult<ParsingResult> {
    let player_items = &state.player.inventory.items;
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

fn pick_item(state: &State, item: Item) -> NRResult<(State, ParsingResult)> {
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

fn drop_item(state: &State, item: Item) -> NRResult<(State, ParsingResult)> {
    let current_room_id = state.current_room;
    if state.player.inventory.items.contains(&item) {
        let (mut new_state, message) = player_remove_item(state, item.clone())?;
        match new_state
            .rooms
            .iter_mut()
            .find(|room| room.id == current_room_id)
        {
            Some(room) => room.stash.add_item(item),
            None => return Err(NoRoom.into()),
        };
        Ok((new_state, ParsingResult::DropItem(message)))
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

fn look_room(state: &State) -> NRResult<ParsingResult> {
    let current_room_id = state.current_room;
    let rooms = state.rooms.clone();
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
