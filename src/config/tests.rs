use crate::{
    config::rooms::Exits,
    util::test_helpers::{self, mock_config, mock_state},
};
#[cfg(test)]
use pretty_assertions::assert_eq;

use super::*;
#[test]
fn it_creates_narratives() {
    let narratives_config_yaml = r"---
          - id: 1
            text: 'This is the first narrative.'
            description: 'This is the first narrative.'";
    assert_eq!(
        vec![Narrative {
            id: 1,
            text: String::from("This is the first narrative."),
            description: String::from("This is the first narrative.")
        }],
        serde_yaml::from_str::<Vec<Narrative>>(narratives_config_yaml).unwrap()
    );
}
#[test]
fn it_creates_items() {
    let items_config = r"---
            - id: 1
              name: sword
              description: a rusty sword
              can_pick: true";
    assert_eq!(
        vec![Item {
            id: 1,
            name: String::from("sword"),
            description: String::from("a rusty sword"),
            can_pick: true
        }],
        serde_yaml::from_str::<Vec<Item>>(items_config).unwrap()
    );
}
#[test]
fn it_creates_allowed_verbs_from_yaml() {
    let allowed_verbs_config = r"---
            - id: 1
              names: 
                - go
              verb_function: normal";
    assert_eq!(
        vec![Verb {
            id: 1,
            names: vec![String::from("go")],
            verb_function: VerbFunction::Normal
        }],
        serde_yaml::from_str::<Vec<Verb>>(allowed_verbs_config).unwrap()
    );
}
#[test]
fn it_creates_subjects() {
    let subject_config = r"---
            - id: 1
              name: text
              description: text
              default_text: text";
    assert_eq!(
        vec![Subject {
            id: 1,
            name: String::from("text"),
            description: String::from("text"),
            default_text: String::from("text")
        }],
        serde_yaml::from_str::<Vec<Subject>>(subject_config).unwrap()
    );
}
#[test]
fn it_creates_room_blueprints() {
    let rooms_config = r"---
          - id: 1
            name: text
            description: text
            exits:
                - room_id: 2
                  direction: south
            item_ids:
                - 1
                - 2
            room_events:
                - 3
            narrative: 2
            subject_ids:
                - 4";

    assert_eq!(
        vec![RoomBlueprint {
            id: 1,
            name: String::from("text"),
            description: String::from("text"),
            exits: vec![Exits {
                room_id: 2,
                direction: directions::Directions::South,
            }],
            item_ids: vec![1, 2],
            // room_events: vec![3],
            narrative: 2,
            subject_ids: vec![4],
        },],
        serde_yaml::from_str::<Vec<RoomBlueprint>>(rooms_config).unwrap()
    );
}
#[test]
fn it_creates_intro() {
    let intro_config = r"---
        text";
    assert_eq!(
        "text".to_string(),
        serde_yaml::from_str::<String>(intro_config).unwrap()
    );
}
#[test]
fn it_creates_events() {
    let events_config = r"---
          - id: 1
            name: text
            description: text
            location: 1
            destination: ~
            narrative: ~
            required_verb: ~
            required_subject: ~
            required_item: ~
            completed: false
            add_item: ~
            remove_old_narrative: false
            remove_item: ~
            required_events: []
            add_subject: ~
            remove_subject: false
            move_subject_to_location: ~
            narrative_after: ~";
    assert_eq!(
        vec![Event {
            id: 1,
            name: String::from("text"),
            description: String::from("text"),
            location: 1,
            destination: None,
            narrative: None,
            required_verb: None,
            required_subject: None,
            required_item: None,
            completed: false,
            add_item: None,
            remove_old_narrative: false,
            remove_item: None,
            required_events: vec![],
            add_subject: None,
            remove_subject: false,
            move_subject_to_location: None,
            narrative_after: None,
        },],
        serde_yaml::from_str::<Vec<Event>>(events_config).unwrap()
    );
}
#[test]
fn it_creates_config_from_default_builder() {
    let config = Config::from_path("fixtures/");
    let config_object = mock_config();
    assert_eq!(config_object, config);
}
#[test]
fn it_creates_state() {
    let config = Config::from_json(&test_helpers::mock_json_data());

    let state = State::init(config);
    let state2 = State::init(Config::from_path("fixtures/"));
    let state_object = mock_state();
    assert_eq!(
        *state.borrow(),
        *state2.borrow(),
        "state and state2 should be the same"
    );
    assert_eq!(
        *state.borrow(),
        state_object,
        "state and state_object should be the same"
    );
    assert_eq!(
        *state2.borrow(),
        state_object,
        "state2 and state_object should be the same"
    );
}
