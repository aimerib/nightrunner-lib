use std::path::Path;

use regex::Regex;

use self::test_helpers::export_json_data;

use super::*;
use crate::config::{directions::Directions, Config, State};
#[cfg(test)]
use pretty_assertions::assert_eq;

#[test]
fn player_takes_item() {
    let config = Config::from_path("fixtures/");
    let state = State::init(config);
    let item = state.config.items[0].clone();
    let item_not_in_room = Item {
        id: 2,
        name: "not in room".to_string(),
        description: "not in room".to_string(),
        can_pick: true,
    };
    let result1 = player_get_item(&state, item);
    let result2 = player_get_item(&state, item_not_in_room);
    assert_eq!(
        result1.unwrap().1,
        ParsingResult::NewItem("\nYou now have a item1\n".to_string())
    );
    assert_eq!(result2.unwrap_err().to_string(), NoItem.to_string());
}
#[test]
fn player_receives_item() {
    let config = Config::from_path("fixtures/");
    let state = State::init(config);
    let item = state.config.items[0].clone();
    let item_not_in_room = Item {
        id: 2,
        name: "not in room".to_string(),
        description: "not in room".to_string(),
        can_pick: true,
    };
    let result1 = player_receive_item(&state, item);
    let result2 = player_receive_item(&state, item_not_in_room);
    assert_eq!(result1.unwrap().1, "\nYou now have a item1\n");
    assert_eq!(result2.unwrap().1, "\nYou now have a not in room\n");
}
#[test]
fn it_removes_player_item() {
    let config = Config::from_path("fixtures/");
    let state = State::init(config);
    let item = state.config.items[0].clone();
    let (new_state, _) = player_receive_item(&state, item.clone()).unwrap();
    let item_not_with_player = Item {
        id: 2,
        name: "not in room".to_string(),
        description: "not in room".to_string(),
        can_pick: true,
    };
    let result1 = player_remove_item(&new_state, item);
    let result2 = player_remove_item(&new_state, item_not_with_player);
    assert_eq!(result1.unwrap().1, "\nYou no longer have a item1\n");
    assert_eq!(result2.unwrap_err().to_string(), NoItem.to_string());
}
#[test]
fn it_moves_player() {
    let config = Config::from_path("fixtures/");
    let state = State::init(config);
    let result1 = move_to_direction(&state, Directions::North).unwrap_err();
    let (new_state, result2) = move_to_direction(&state, Directions::South).unwrap();
    let result3 = move_to_direction(&new_state, Directions::East).unwrap_err();
    let (_, result4) = move_to_direction(&new_state, Directions::North).unwrap();
    assert_eq!(result1.to_string(), InvalidMovement.to_string());
    assert_eq!(result2, MoveSuccess);
    assert_eq!(result3.to_string(), InvalidMovement.to_string());
    assert_eq!(result4, MoveSuccess);
}
#[test]
fn it_parses_templated_narratives() {
    let re = Regex::new(r"\{(.*?)\}").unwrap();
    let config = Config::from_path("fixtures/");
    let state = State::init(config);
    let room = state
        .rooms
        .iter()
        .find(|room| room.id == 2)
        .unwrap()
        .clone();
    let narrative = state
        .config
        .narratives
        .iter()
        .find(|n| n.id == room.narrative)
        .unwrap()
        .clone();
    let raw_room_text = &narrative.text;
    let captures = raw_room_text
        .clone()
        .lines()
        .map(|sentence| {
            let templated_word_captures: TemplateCaptures = re
                .captures_iter(sentence)
                .map(|cap| {
                    let start = &cap.get(0).unwrap().start();
                    let end = &cap.get(0).unwrap().end();
                    let text = cap.get(1).unwrap().as_str();
                    (start.to_owned(), end.to_owned(), text.to_owned())
                })
                .collect();
            templated_word_captures
        })
        .collect::<Vec<_>>();
    let capture1 = captures[0].clone();
    let capture2 = captures[1].clone();
    let capture3 = captures[2].clone();
    assert_eq!(
        capture1.captures[0].to_string(),
        "start: 45, end: 52, text: item3"
    );
    assert_eq!(
        TemplateCapture {
            start: 45,
            end: 52,
            text: "item3".to_string(),
        },
        capture1.captures[0],
    );
    assert_eq!(capture2.captures.len(), 0);
    assert_eq!(
        capture3.captures[0].to_string(),
        "start: 52, end: 62, text: subject2"
    );
    assert_eq!(
        TemplateCaptures {
            captures: vec![TemplateCapture {
                start: 52,
                end: 62,
                text: "subject2".to_string(),
            },],
        },
        capture3,
    );
}

#[test]
fn it_parses_room_text() {
    let config = Config::from_path("fixtures/");
    let mut state = State::init(config);
    let narrative_text = String::from(
        "this is a templated which exists in the game {item1}.\n\nthis is a templated subject that exists in the game {subject1}.",
    );
    let mut message_parts = HashMap::new();
    message_parts.insert(MessageParts::EventText, String::from(""));
    message_parts.insert(MessageParts::RoomText, "this is a templated which exists in the game item1.\n\nthis is a templated subject that exists in the game subject1.".to_string());
    message_parts.insert(
        MessageParts::Exits,
        String::from("Exits:\nto the south you see second room"),
    );
    // with either an item or subject in the room, it returns the templated text along with the
    // narrative provided and the exits information for display.
    let mut result = parse_room_text(&state, narrative_text.clone(), "".to_string(), None);
    assert_eq!(
        result.unwrap(),
        EventMessage {
            message:
            "this is a templated which exists in the game item1.\n\nthis is a templated subject that exists in the game subject1.\n\n\nExits:\nto the south you see second room"
            .to_string(),
            templated_words: vec!["item1".to_string(), "subject1".to_string()],
            message_parts: message_parts.clone(),
        }
    );
    // room two doesn't contain the items templated in the narrative
    state.current_room = 2;
    // so here we expect it to return just the narrative and the exits information for display.
    result = parse_room_text(&state, narrative_text, "".to_string(), None);
    message_parts.insert(
        MessageParts::Exits,
        String::from("Exits:\nto the north you see first room"),
    );
    assert_eq!(
        result.unwrap(),
        EventMessage {
            message:
            "this is a templated which exists in the game item1.\n\nthis is a templated subject that exists in the game subject1.\n\n\nExits:\nto the north you see first room"
                    .to_string(),
            templated_words: vec![],
            message_parts
        }
    );
}
#[test]
#[ignore]
fn it_generates_json_file_and_clean_up() {
    export_json_data();
    let path = Path::new("test.json");
    // check if the file exists
    assert_eq!(path.exists(), true);
}
