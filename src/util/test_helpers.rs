use crate::config::{
    determiners::AllowedDeterminers,
    directions::{AllowedDirections, Directions},
    movements::AllowedMovements,
    prepositions::AllowedPrepositions,
    rooms::{Exits, RoomBlueprint},
    Config, Event, Item, Narrative, State, Subject, Verb, VerbFunction,
};

/// Returns a sample Config that can be used during testing.
pub fn mock_config() -> Config {
    Config {
        allowed_prepositions: AllowedPrepositions::init(),
        allowed_determiners: AllowedDeterminers::init(),
        allowed_directions: AllowedDirections::init(),
        allowed_movements: AllowedMovements::init(),
        intro: "The introduction text to be displayed at the begining of the game.".to_string(),
        allowed_verbs: vec![
            Verb {
                id: 1,
                names: vec![String::from("quit"), String::from(":q"), String::from("q")],
                verb_function: VerbFunction::Quit,
            },
            Verb {
                id: 2,
                names: vec![String::from("help")],
                verb_function: VerbFunction::Help,
            },
            Verb {
                id: 3,
                names: vec![String::from("look"), String::from("stare")],
                verb_function: VerbFunction::Look,
            },
            Verb {
                id: 4,
                names: vec![String::from("inventory"), String::from("i")],
                verb_function: VerbFunction::Inventory,
            },
            Verb {
                id: 5,
                names: vec![
                    String::from("pick"),
                    String::from("take"),
                    String::from("grab"),
                    String::from("pi"),
                    String::from("tk"),
                    String::from("gr"),
                    String::from("get"),
                    String::from("g"),
                ],
                verb_function: VerbFunction::Take,
            },
            Verb {
                id: 6,
                names: vec![String::from("drop"), String::from("place")],
                verb_function: VerbFunction::Drop,
            },
            Verb {
                id: 7,
                names: vec![String::from("give"), String::from("hand")],
                verb_function: VerbFunction::Normal,
            },
            Verb {
                id: 8,
                names: vec![String::from("talk"), String::from("chat")],
                verb_function: VerbFunction::Talk,
            },
            Verb {
                id: 9,
                names: vec![String::from("hug")],
                verb_function: VerbFunction::Normal,
            },
        ],
        items: vec![
            Item {
                id: 1,
                name: String::from("item1"),
                description: String::from("item 1 description"),
                can_pick: false,
            },
            Item {
                id: 2,
                name: String::from("item2"),
                description: String::from("item 2 description"),
                can_pick: true,
            },
            Item {
                id: 3,
                name: String::from("item3"),
                description: String::from("item 3 description"),
                can_pick: true,
            }
        ],
        narratives: vec![
            Narrative {
                id: 1,
                text: String::from("text"),
                description: String::from("text"),
            },
            Narrative {
                id: 2,
                text: String::from(
                    "this is a templated which exists in the game {item3}.\n\nthis is a templated subject that exists in the game {subject2}.",
                ),
                description: String::from("text"),
            },
            Narrative {
                id: 3,
                text: String::from("this narrative should replace the old one."),
                description: String::from("a replaced narrative"),
            },
            Narrative {
                id: 4,
                text: String::from("this narrative should be returned along with the text of room 1."),
                description: String::from("a narrative that is added to the room narrative"),
            },
            Narrative {
              id: 5,
              text: "this narrative should be returned along with the text of room 1 when completing event 6.".to_string(),
              description: "a narrative that is added to the room narrative".to_string()
            },
        ],

        room_blueprints: vec![
            RoomBlueprint {
                id: 1,
                name: String::from("room 1"),
                description: String::from("first room"),
                exits: vec![Exits {
                    room_id: 2,
                    direction: Directions::South,
                }],
                item_ids: vec![1, 2],
                narrative: 1,
                subject_ids: vec![1],
            },
            RoomBlueprint {
                id: 2,
                name: String::from("room 2"),
                description: String::from("second room"),
                exits: vec![Exits {
                    room_id: 1,
                    direction: Directions::North,
                }],
                item_ids: vec![3],
                narrative: 2,
                subject_ids: vec![2],
            },
        ],
        events: vec![
            Event {
                id: 1,
                name: String::from("text"),
                description: String::from("text"),
                location: 1,
                destination: None,
                narrative: Some(1),
                required_verb: Some(2),
                required_subject: Some(1),
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
            },
            Event {
                id: 2,
                name: "event 2".to_string(),
                description: "hug subject 2 - requires event 4".to_string(),
                location: 1,
                destination: None,
                narrative: Some(3),
                required_verb: Some(9),
                required_subject: Some(1),
                required_item: None,
                completed: false,
                add_item: None,
                remove_old_narrative: true,
                remove_item: None,
                required_events: vec![4],
                add_subject: None,
                remove_subject: false,
                move_subject_to_location: None,
                narrative_after: None,
            },
            Event {
                id: 3,
                name: String::from("text"),
                description: String::from("text"),
                location: 1,
                destination: None,
                narrative: Some(2),
                required_verb: Some(2),
                required_subject: Some(1),
                required_item: None,
                completed: false,
                add_item: None,
                remove_old_narrative: true,
                remove_item: None,
                required_events: vec![2],
                add_subject: None,
                remove_subject: false,
                move_subject_to_location: None,
                narrative_after: None,
            },
            Event {
                id: 4,
                name: String::from("event 4"),
                description: String::from("talk to subject 1"),
                location: 1,
                destination: None,
                narrative: Some(1),
                required_verb: Some(8),
                required_subject: Some(1),
                required_item: None,
                completed: false,
                add_item: None,
                remove_old_narrative: true,
                remove_item: None,
                required_events: vec![],
                add_subject: None,
                remove_subject: false,
                move_subject_to_location: None,
                narrative_after: None,
            },
            Event {
                id: 5,
                name: "event 5".to_string(),
                description: "gives item 2 to player when talking to subject2".to_string(),
                location: 2,
                destination: Some(1),
                narrative: Some(4),
                required_verb: Some(8),
                required_subject: Some(2),
                required_item: None,
                completed: false,
                add_item: Some(2),
                remove_old_narrative: false,
                remove_item: None,
                required_events: vec![],
                add_subject: None,
                remove_subject: false,
                move_subject_to_location: None,
                narrative_after: None,
            },
            Event {
                id: 6,
                name: "event 6".to_string(),
                description: "gives item 2 to subject1 when talking to subject1 after event 5".to_string(),
                location: 1,
                destination: None,
                narrative: Some(4),
                required_verb: Some(7),
                required_subject: Some(1),
                required_item: Some(2),
                completed: false,
                add_item: None,
                remove_old_narrative: false,
                remove_item: Some(2),
                required_events: vec![5],
                add_subject: None,
                remove_subject: false,
                move_subject_to_location: None,
                narrative_after: None,
            }
        ],

        subjects: vec![
            Subject {
                id: 1,
                name: String::from("subject1"),
                description: String::from("a subject description"),
                default_text: String::from("default text"),
            },
            Subject {
                id: 2,
                name: String::from("subject2"),
                description: String::from("subject2 description"),
                default_text: String::from("default text"),
            }
        ],
    }
}
/// function to create sample JSON data for testing
pub fn mock_json_data() -> String {
    let data = mock_config();
    serde_json::to_string(&data).unwrap()
}

/// export json data to a file
pub fn export_json_data() {
    let data = mock_json_data();
    std::fs::write("test.json", data).unwrap();
}

/// function to create sample State strcuture for testing
pub fn mock_state() -> State {
    State::init(mock_config())
}
