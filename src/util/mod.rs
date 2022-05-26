use std::collections::HashMap;
use std::fmt::{self, Display};
use std::iter::FromIterator;

use regex::Regex;
use serde::{Deserialize, Serialize};
/// Module containing a few utility functions to
/// make testing a little easier.
pub mod test_helpers;

use crate::config::directions::Directions;
// use crate::config::rooms::Room;
use crate::config::{Item, State};
use crate::parser::errors::{InvalidMovement, InvalidRoom, NoItem, NoRoom};
use crate::parser::interpreter::{EventMessage, MessageParts};
use crate::NRResult;
use crate::ParsingResult;

/// This struct is used when parsing the narrative text.
/// Narrative texts can contain template strings, which are replaced with
/// their corresponding text along with the beginning and end indices of the
/// template string.
#[derive(Debug, Clone, PartialEq)]
struct TemplateCapture {
    start: usize,
    end: usize,
    text: String,
}

#[derive(Debug, Clone, PartialEq)]
struct TemplateCaptures {
    captures: Vec<TemplateCapture>,
}

struct TemplateCapturesIter(TemplateCaptures);

impl TemplateCaptures {
    fn iter(self) -> TemplateCapturesIter {
        TemplateCapturesIter(self)
    }
}

impl FromIterator<(usize, usize, String)> for TemplateCaptures {
    fn from_iter<I: IntoIterator<Item = (usize, usize, String)>>(iter: I) -> Self {
        let mut capture_vec = Vec::new();
        for i in iter {
            capture_vec.push(TemplateCapture {
                start: i.0,
                end: i.1,
                text: i.2,
            });
        }
        TemplateCaptures {
            captures: capture_vec,
        }
    }
}

impl fmt::Display for TemplateCapture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "start: {}, end: {}, text: {}",
            self.start, self.end, self.text
        )
    }
}

impl Iterator for TemplateCapturesIter {
    type Item = TemplateCapture;
    fn next(&mut self) -> Option<Self::Item> {
        let next_capture = self.0.captures.pop();
        match next_capture {
            Some(capture) => Some(TemplateCapture {
                start: capture.start,
                end: capture.end,
                text: capture.text,
            }),
            None => None,
        }
    }
}

impl IntoIterator for TemplateCaptures {
    type Item = TemplateCapture;
    type IntoIter = TemplateCapturesIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// This function is used when the player issues a command
/// to take an item.
///
/// If the Item is not found in the room, then a ParsingError is returned
/// with a message indicating that the item was not found.
/// Otherwise, the item is removed from the room and added to the player's
/// inventory and a ParsingResult is returned with a message indicating that
/// the item was taken.
pub fn player_get_item(state: &State, item: Item) -> NRResult<(State, ParsingResult)> {
    let mut new_state = state.clone();
    let current_room_id = new_state.current_room;
    let current_room = new_state
        .rooms
        .iter_mut()
        .find(|room| room.id == current_room_id)
        .unwrap();

    match current_room.stash.remove_item(item) {
        Ok(item) => {
            new_state.player.inventory.add_item(item.clone());
            let message = format!("\nYou now have a {}\n", item.name);
            Ok((new_state, ParsingResult::NewItem(message)))
        }
        Err(_) => Err(NoItem.into()),
    }
}

/// This function is used when the player is given an item.
/// This function is called by the events parser if the event
/// indicates that the player should receive an item.
pub fn player_receive_item(state: &State, item: Item) -> NRResult<(State, String)> {
    let mut new_state = state.clone();
    new_state.player.inventory.add_item(item.clone());
    let item_message = format!("\nYou now have a {}\n", item.name);
    Ok((new_state, item_message))
}

/// This function is used to remove an item from the player's inventory
/// but it won't add the inventory to the room. This is used when the
/// event indicates that the player should lose an item.
pub fn player_remove_item(state: &State, item: Item) -> NRResult<(State, String)> {
    let mut new_state = state.clone();
    let player = &mut new_state.player;
    let old_item = player.inventory.remove_item(item)?;
    Ok((
        new_state,
        format!("\nYou no longer have a {}\n", old_item.name),
    ))
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

/// Represents a successful movement.
pub struct MoveSuccess;

impl Display for MoveSuccess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "You moved in that direction")
    }
}

/// This function is used when the player attempts to move in a direction.
/// If the direction given doesn't exist, then a
/// `ParsingError::InvalidMovement(MoveError::NoExit)` is returned.
/// If the player can move in the direction, then the player's current room
/// is updated and a `ParsingResult::Movement(MoveSuccess)` is returned.
pub fn move_to_direction(state: &State, direction: Directions) -> NRResult<(State, MoveSuccess)> {
    let mut new_state = state.clone();
    let current_room_id = new_state.current_room;
    if let Some(current_room) = new_state
        .rooms
        .iter_mut()
        .find(|room| room.id == current_room_id)
    {
        if let Ok(room_id) = current_room.can_move(direction) {
            new_state.current_room = room_id;
            Ok((new_state, MoveSuccess))
        } else {
            Err(InvalidMovement.into())
        }
    } else {
        Err(NoRoom.into())
    }
}

/// Displays the help message for the player.
pub fn display_help(state: &State) -> NRResult<ParsingResult> {
    let valid_verbs = state.config.allowed_verbs.clone();
    let valid_verbs_string = &valid_verbs
        .iter()
        .map(|verb| verb.names[0].clone())
        .collect::<Vec<String>>()
        .join(", ")[..];
    let help_text: String = "
To play this game you type your commands and hit enter to execute them. Typically a command \
  has at most three parts: a verb, a subject, and an item. A verb indicates an action you, \
  the player, wants to execute. Many commands can be executed with just a verb such as look, \
  help, quit. For more complex commands you will also need verb and either a subject or an \
  item. A command can also have a verb, item, and subject. A complex command can be: look at \
  dog, talk to person, pick the box, give the box to the dog.

The game will ignore words like 'to', 'the', 'at', 'from', so using them is optional. A \
  valid command can be: talk person, pick box, go south, climb tree, use axe tree.

Valid verbs: "
        .to_string()
        + valid_verbs_string;
    Ok(ParsingResult::Help(help_text))
}

/// This function is used to return the text
/// that should be displayed to the player in the room.
/// This text is composed of the current narrative associated
/// with this room, and the exits that are available to the player.
///
/// This function only returns highlting information if either the
/// item or the subject to be highlighted is in the room, otherwise
/// it returns the narrative withouth the template.
///
/// The items and subjects vectors are used to determine if the
/// corresponding items and subjects should be returned with the
/// room text. These can be used for highlighting the items and
/// subjects in the front-end.
///
/// This function will return a Result wrapping an EventMessage
/// with the following format:
/// ```rust
/// # use nightrunner_lib::parser::interpreter::{EventMessage, MessageParts};
/// # use std::collections::HashMap;
/// let mut message_parts = HashMap::new();
/// message_parts.insert(MessageParts::RoomText, "some message with highlighted text.".to_string());
/// message_parts.insert(MessageParts::RoomText, "You now have item1.".to_string());
/// message_parts.insert(MessageParts::RoomText, "Exits: to the south you see an alley.".to_string());
/// let event_message = EventMessage {
///     message: "some message with highlighted text.\nYou now have item1.\nExits: to the south you see an alley.".to_string(),
///     templated_words: vec!["highlighted".to_string()],
///     message_parts: message_parts,
/// };
/// ```
/// and the parser will return a respone wrappping this result.
/// `message_parts` contains the three parts of the message that
/// is returned and can be used for layouting the message in the
/// front-end, otherwise the message field can be used for simpler
/// applications.
pub fn parse_room_text(
    state: &State,
    narrative_text: String,
    event_message: String,
    event_id: Option<u16>,
) -> NRResult<EventMessage> {
    let current_room = match state
        .rooms
        .iter()
        .find(|room| room.id == state.current_room)
    {
        Some(room) => room,
        None => return Err(InvalidRoom.into()),
    };
    let player_items = state
        .player
        .inventory
        .items
        .clone()
        .iter()
        .map(|item| item.name.clone())
        .collect::<Vec<String>>();
    let room_items = current_room
        .stash
        .items
        .clone()
        .iter()
        .map(|item| item.name.clone())
        .collect::<Vec<_>>();
    let room_subjects = current_room
        .subjects
        .clone()
        .iter()
        .map(|subject| subject.name.clone())
        .collect::<Vec<_>>();
    let mut event_items = vec![];
    let event = state
        .config
        .events
        .iter()
        .find(|event| Some(event.id) == event_id);
    if let Some(event) = event {
        if let Some(item_id) = event.add_item {
            if let Some(item) = state.config.items.iter().find(|item| item.id == item_id) {
                event_items.push(item.name.clone())
            };
        }
        if let Some(item_id) = event.remove_item {
            if let Some(item) = state.config.items.iter().find(|item| item.id == item_id) {
                event_items.push(item.name.clone())
            };
        }
    };

    let exits_vec = current_room
        .exits
        .clone()
        .iter()
        .map(
            |exit| match state.rooms.iter().find(|room| room.id == exit.room_id) {
                Some(room) => format!(
                    "to the {} you see {}",
                    exit.direction.clone(),
                    room.description.clone()
                ),
                None => String::new(),
            },
        )
        .collect::<Vec<String>>();
    let exits_string = match exits_vec.len() {
        0 => String::new(),
        _ => String::from("Exits:\n") + &exits_vec.join("\n")[..],
    };
    let items_and_subjects = player_items
        .iter()
        .chain(room_items.iter())
        .chain(event_items.iter())
        .chain(room_subjects.iter())
        .cloned()
        .collect::<Vec<_>>();
    let (room_text, templated_words_room) =
        process_templated_text(narrative_text, &items_and_subjects);
    let (event_text, templated_words_event) =
        process_templated_text(event_message, &items_and_subjects);
    let mut message_parts = HashMap::new();
    message_parts.insert(MessageParts::RoomText, room_text.clone());
    message_parts.insert(MessageParts::Exits, exits_string.clone());
    message_parts.insert(MessageParts::EventText, event_text.clone());
    let message = room_text + "\n" + event_text.as_str() + "\n\n" + exits_string.as_str();
    let mut templated_words = templated_words_room
        .iter()
        .chain(templated_words_event.iter())
        .cloned()
        .collect::<Vec<String>>();
    templated_words.sort_unstable();
    templated_words.dedup();
    Ok(EventMessage {
        message,
        message_parts,
        templated_words,
    })
}

fn process_templated_text(text: String, items_and_subjects: &[String]) -> (String, Vec<String>) {
    let mut templated_words: Vec<String> = Vec::new();
    let processed_text = text
        .lines()
        .map(|sentence| {
            let mut extracted_text = sentence.to_string();
            let re = Regex::new(r"\{(.*?)\}").unwrap();
            let templated_word_captures: TemplateCaptures = re
                .captures_iter(sentence)
                .map(|cap| {
                    let start = &cap.get(0).unwrap().start();
                    let end = &cap.get(0).unwrap().end();
                    let text = cap.get(1).unwrap().as_str();
                    (start.to_owned(), end.to_owned(), text.to_owned())
                })
                .collect();

            let capture_length = templated_word_captures.captures.len();
            if capture_length > 0 {
                for capture in templated_word_captures {
                    if items_and_subjects.contains(&capture.text.to_string()) {
                        templated_words.push(capture.text.clone());
                        extracted_text = extracted_text.clone()[..capture.start].to_string()
                            + &capture.text
                            + &extracted_text.clone()[capture.end..];
                    } else {
                        extracted_text = extracted_text.clone()[..capture.start].to_string()
                            + &capture.text
                            + &extracted_text.clone()[capture.end..];
                    }
                }
                extracted_text
            } else {
                sentence.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("\n");
    (processed_text, templated_words)
}

#[cfg(test)]
mod tests;
