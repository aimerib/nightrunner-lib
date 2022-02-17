use crate::config::Config;
#[cfg(test)]
use pretty_assertions::assert_eq;

use super::*;
#[test]
fn it_extracts_input_verb() {
    let config = Config::from_path("fixtures/");
    let state = State::init(config);
    let verb = extract_verb(&*state.borrow(), &vec!["look".to_string()]);
    assert_eq!(verb.unwrap().names[0], "look");
}
#[test]
fn it_extracts_input_subject() {
    let config = Config::from_path("fixtures/");
    let state = State::init(config);
    let subject = extract_subject(
        &*state.borrow(),
        &vec!["look".to_string(), "subject1".to_string()],
    );
    assert_eq!(subject.unwrap().name, "subject1");
}
#[test]
fn it_extracts_input_movement() {
    let config = Config::from_path("fixtures/");
    let state = State::init(config);
    let movement1 = extract_movement(
        &*state.borrow(),
        &vec!["go".to_string(), "north".to_string()],
    );
    let movement2 = extract_movement(&*state.borrow(), &vec!["south".to_string()]);
    assert_eq!(movement1.unwrap(), Directions::NORTH);
    assert_eq!(movement2.unwrap(), Directions::SOUTH);
}
#[test]
fn it_extracts_input_item() {
    let config = Config::from_path("fixtures/");
    let state = State::init(config);
    let item1 = extract_item(
        &*state.borrow(),
        &vec!["take".to_string(), "item1".to_string()],
        "take item1",
    );
    let item2 = extract_item(
        &*state.borrow(),
        &vec!["look".to_string(), "item2".to_string()],
        "look item2",
    );
    assert_eq!(item1.unwrap().name, "item1");
    assert_eq!(item2.unwrap().name, "item2");
}
#[test]
fn parse_action_fn_parses_correctly() {
    let config = Config::from_path("fixtures/");
    let state = State::init(config);
    let action1 = parse_action(
        &*state.borrow(),
        vec!["take".to_string(), "item1".to_string()],
        "take item1",
    );
    let action2 = parse_action(
        &*state.borrow(),
        vec!["look".to_string(), "subject1".to_string()],
        "look subject1",
    );
    let action3 = parse_action(
        &*state.borrow(),
        vec!["go".to_string(), "north".to_string()],
        "go north",
    );
    assert!(action1.is_valid());
    assert!(action1.verb.unwrap().names.contains(&"take".to_string()));
    assert_eq!(action1.item.unwrap().name, "item1");
    assert!(action2.is_valid());
    assert!(action2.verb.unwrap().names.contains(&"look".to_string()));
    assert_eq!(action2.subject.unwrap().name, "subject1");
    assert!(action3.is_valid());
    assert_eq!(action3.verb, None);
    assert_eq!(action3.movement.unwrap(), Directions::NORTH);
}
#[test]
fn it_parses_action() {
    let config = Config::from_path("fixtures/");
    let state = State::init(config);
    let action = Action::parse(&*state.borrow(), "take item1");
    assert!(action.is_valid());
    assert!(action
        .verb
        .clone()
        .unwrap()
        .names
        .contains(&"take".to_string()));
    assert_eq!(action.item.clone().unwrap().name, "item1");
    assert_eq!(action.action_type(), ActionType::VerbItem);
    assert_eq!(format!("{}", action), "take item1");
}
