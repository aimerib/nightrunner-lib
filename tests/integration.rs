use std::collections::HashMap;

#[cfg(test)]
use pretty_assertions::assert_eq;

use nightrunner_lib::{
    parser::interpreter::{MessageParts, ParsingResult},
    NightRunnerBuilder,
};
#[test]
fn it_works_with_path_to_configs() {
    let nr = NightRunnerBuilder::new()
        .with_path_for_config("fixtures/")
        .build();

    let result = nr.parse_input("look");
    assert_eq!(
        result.unwrap(),
        ParsingResult::Look("first room\nHere you see: \n\na item1\na item2".to_string())
    );

    let result_json = nr.json_parse_input("look");
    assert_eq!(
        result_json,
        r#"{"ok":{"look":"first room\nHere you see: \n\na item1\na item2"}}"#
    );
}
#[test]
fn it_works_with_json_data() {
    use nightrunner_lib::parser::interpreter::EventMessage;
    let data = nightrunner_lib::util::test_helpers::mock_json_data();
    let nr = NightRunnerBuilder::new().with_json_data(&data).build();
    let mut message_parts = HashMap::new();

    message_parts.insert(
        MessageParts::RoomText,
        "this is a templated which exists in the game item1.\n\nthis is a templated subject that exists in the game subject1.".to_string(),
    );
    message_parts.insert(MessageParts::EventText, "".to_string());
    message_parts.insert(
        MessageParts::Exits,
        "Exits:\nto the north you see first room".to_string(),
    );
    let mut result = nr.parse_input("look");
    assert_eq!(
        result.unwrap(),
        ParsingResult::Look("first room\nHere you see: \n\na item1\na item2".to_string())
    );
    result = nr.parse_input("south");
    assert_eq!(
        result.unwrap(),
        ParsingResult::EventSuccess(
                    EventMessage {
                        message: "this is a templated which exists in the game item1.\n\nthis is a templated subject that exists in the game subject1.\n\n\nExits:\nto the north you see first room".to_string(),
                        templated_words: vec![],
                        message_parts: message_parts.clone()
                    })
    );
    result = nr.parse_input("look");
    assert_eq!(
        result.unwrap(),
        ParsingResult::Look("second room".to_string())
    );
    result = nr.parse_input("talk subject2");
    message_parts.insert(
        MessageParts::RoomText,
        "this is a templated which exists in the game item1.\n\nthis is a templated subject that exists in the game subject1.\n\nthis narrative should be returned along with the text of room 1.".to_string(),
    );
    message_parts.insert(
        MessageParts::EventText,
        "\nYou now have a item2".to_string(),
    );
    assert_eq!(
        result.unwrap(),
        ParsingResult::EventSuccess(
                    EventMessage {
                        message: "this is a templated which exists in the game item1.\n\nthis is a templated subject that exists in the game subject1.\n\nthis narrative should be returned along with the text of room 1.\n\nYou now have a item2\n\nExits:\nto the north you see first room".to_string(),
                        templated_words: vec![],
                        message_parts: message_parts.clone()
                    })
    );
    result = nr.parse_input("give item2 subject1");
    message_parts.insert(
        MessageParts::RoomText,
        "text\n\nthis narrative should be returned along with the text of room 1.".to_string(),
    );
    message_parts.insert(
        MessageParts::EventText,
        "\nYou no longer have a item2".to_string(),
    );
    message_parts.insert(
        MessageParts::Exits,
        "Exits:\nto the south you see second room".to_string(),
    );
    assert_eq!(
        result.unwrap(),
        ParsingResult::EventSuccess(
                    EventMessage {
                        message: "text\n\nthis narrative should be returned along with the text of room 1.\n\nYou no longer have a item2\n\nExits:\nto the south you see second room".to_string(),
                        templated_words: vec![],
                        message_parts
                    })
    );

    let result_json = nr.json_parse_input("look");
    assert_eq!(
        result_json,
        r#"{"ok":{"look":"first room\nHere you see: \n\na item1\na item2"}}"#
    );
}
