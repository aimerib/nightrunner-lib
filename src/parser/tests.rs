use std::collections::HashMap;

use super::*;
use crate::{
    config::{Config, State},
    parser::{
        errors::InvalidEvent,
        interpreter::{EventMessage, MessageParts},
    },
};
#[cfg(test)]
use pretty_assertions::assert_eq;

#[test]
fn it_parses_single_verb() {
    let config = Config::from_path("fixtures/");
    let state = State::init(config);
    let mut result = parse(state.clone(), "quit");
    assert_eq!(result.unwrap(), ParsingResult::Quit);

    result = parse(state.clone(), "look");
    assert_eq!(
        result.unwrap(),
        ParsingResult::Look("first room\nHere you see: \n\na item1\na item2".to_string())
    );
}
#[test]
fn it_parses_verb_and_item_or_subject() {
    let config = Config::from_path("fixtures/");
    let state = State::init(config);
    let mut result = parse(state.clone(), "take item2");
    let mut message_parts = HashMap::new();
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        ParsingResult::NewItem("\nYou now have a item2\n".to_string())
    );
    assert_eq!(
        state.borrow().player.inventory.items[0].name,
        "item2".to_string()
    );

    result = parse(state.clone(), "talk subject1");
    message_parts.insert(MessageParts::RoomText, "text".to_string());
    message_parts.insert(MessageParts::EventText, "".to_string());
    message_parts.insert(
        MessageParts::Exits,
        "Exits:\nto the south you see second room".to_string(),
    );
    assert_eq!(
        result.unwrap(),
        ParsingResult::EventSuccess(EventMessage {
            message: "text\n\n\nExits:\nto the south you see second room".to_string(),
            templated_words: vec![],
            message_parts: message_parts.clone(),
        })
    );

    result = parse(state.clone(), "go south");
    message_parts.insert(MessageParts::RoomText, "this is a templated which exists in the game item1.\n\nthis is a templated subject that exists in the game subject1.".to_string());
    message_parts.insert(MessageParts::EventText, "".to_string());
    message_parts.insert(
        MessageParts::Exits,
        "Exits:\nto the north you see first room".to_string(),
    );
    assert_eq!(
        result.unwrap(),
        ParsingResult::EventSuccess(EventMessage {
            message: "this is a templated which exists in the game item1.\n\nthis is a templated subject that exists in the game subject1.\n\n\nExits:\nto the north you see first room".to_string(),
            templated_words: vec![],
            message_parts,
        })
    );

    result = parse(state.clone(), "give item2 to subject2");
    // There is no event for player giving item2 to subject2
    // so we expect an error. InvalidEvent should be used to
    // indicate that the event is not valid, and how to handle
    // this error is up to the front-end. Perhaps you display
    // a message to the user saying that this action is invalid,
    // or you don't understand the command.
    // For convenience, this error wraps the action as it was
    // interpreted by the parser from the imput. This is useful
    // when writing custom logic for the front-end.
    assert_eq!(result.unwrap_err().to_string(), InvalidEvent.to_string());
}
