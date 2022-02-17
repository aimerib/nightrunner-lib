//! This library is a text-adventure game engine that can be used to create
//! text based adventure games. It is designed to be used with a front-end
//! which can be written in any language. Implementing this library in a
//! language is a matter of writing a front-end an passing string data to
//! the library for parsing.
//!
//! The configuration of the game is done in the `Config` struct
//! and can be initialized both with YAML files and serialized
//! JSON data, so it is perfect for both web and desktop games.
//!
//! The `parse_input` and `parse_input_json` functions are the only
//! functions that need to be called by the front-end, but the library
//! exposes some of the internal structs and functions to help developers
//! understand how the library works, and to allow a little bit of flexibility
//! in how the library is used.
//!
//! # Example:
//! ```rust
//! use nightrunner_lib::NightRunner;
//! use nightrunner_lib::NightRunnerBuilder;
//! use nightrunner_lib::parser::interpreter::{ParsingResult};
//! let data = r#"{
//!   "allowed_verbs": [
//!     {
//!       "id": 1,
//!       "names": [
//!         "quit",
//!         ":q",
//!         "q"
//!       ],
//!       "verb_function": "quit"
//!     },
//!     {
//!       "id": 7,
//!       "names": [
//!         "give",
//!         "hand"
//!       ],
//!       "verb_function": "normal"
//!     },
//!     {
//!       "id": 2,
//!       "names": [
//!         "help"
//!       ],
//!       "verb_function": "help"
//!     },
//!     {
//!       "id": 3,
//!       "names": [
//!         "look",
//!         "stare"
//!       ],
//!       "verb_function": "look"
//!     },
//!     {
//!       "id": 4,
//!       "names": [
//!         "inventory",
//!         "i"
//!       ],
//!       "verb_function": "inventory"
//!     },
//!     {
//!       "id": 6,
//!       "names": [
//!         "drop",
//!         "place"
//!       ],
//!       "verb_function": "drop"
//!     },
//!     {
//!       "id": 8,
//!       "names": [
//!         "talk",
//!         "chat"
//!       ],
//!       "verb_function": "talk"
//!     },
//!     {
//!       "id": 5,
//!       "names": [
//!         "pick",
//!         "take",
//!         "grab",
//!         "pi",
//!         "tk",
//!         "gr",
//!         "get",
//!         "g"
//!       ],
//!       "verb_function": "take"
//!     },
//!     {
//!       "id": 9,
//!       "names": [
//!         "hug"
//!       ],
//!       "verb_function": "normal"
//!     }
//!   ],
//!   "items": [
//!     {
//!       "id": 1,
//!       "name": "item1",
//!       "description": "item 1 description",
//!       "can_pick": false
//!     },
//!     {
//!       "id": 2,
//!       "name": "item2",
//!       "description": "item 2 description",
//!       "can_pick": true
//!     }
//!   ],
//!   "subjects": [
//!     {
//!       "id": 1,
//!       "name": "subject1",
//!       "description": "a subject description",
//!       "default_text": "default text"
//!     }
//!   ],
//!   "narratives": [
//!     {
//!       "id": 1,
//!       "text": "text",
//!       "description": "text"
//!     },
//!     {
//!       "id": 2,
//!       "text": "this is a templated which exists in the game {item1}.\n\nthis is a templated subject that exists in the game {subject1}.",
//!       "description": "text"
//!     },
//!     {
//!       "id": 3,
//!       "text": "this narrative should replace the old one.",
//!       "description": "a replaced narrative"
//!     }
//!   ],
//!   "events": [
//!     {
//!       "id": 1,
//!       "name": "text",
//!       "description": "text",
//!       "location": 1,
//!       "destination": null,
//!       "narrative": 1,
//!       "required_verb": 2,
//!       "required_subject": 1,
//!       "required_item": null,
//!       "completed": false,
//!       "add_item": null,
//!       "remove_old_narrative": false,
//!       "remove_item": null,
//!       "required_events": []
//!     },
//!     {
//!       "id": 2,
//!       "name": "text",
//!       "description": "text",
//!       "location": 1,
//!       "destination": null,
//!       "narrative": 3,
//!       "required_verb": 9,
//!       "required_subject": 1,
//!       "required_item": null,
//!       "completed": false,
//!       "add_item": null,
//!       "remove_old_narrative": true,
//!       "remove_item": null,
//!       "required_events": [
//!         4
//!       ]
//!     },
//!     {
//!       "id": 3,
//!       "name": "text",
//!       "description": "text",
//!       "location": 1,
//!       "destination": null,
//!       "narrative": 2,
//!       "required_verb": 2,
//!       "required_subject": 1,
//!       "required_item": null,
//!       "completed": false,
//!       "add_item": null,
//!       "remove_old_narrative": true,
//!       "remove_item": null,
//!       "required_events": [
//!         2
//!       ]
//!     },
//!     {
//!       "id": 4,
//!       "name": "text",
//!       "description": "text",
//!       "location": 1,
//!       "destination": null,
//!       "narrative": 1,
//!       "required_verb": 8,
//!       "required_subject": 1,
//!       "required_item": null,
//!       "completed": false,
//!       "add_item": null,
//!       "remove_old_narrative": true,
//!       "remove_item": null,
//!       "required_events": []
//!     }
//!   ],
//!   "intro": "text",
//!   "rooms": [
//!     {
//!       "id": 1,
//!       "name": "room 1",
//!       "description": "first room",
//!       "exits": [
//!         {
//!           "room_id": 2,
//!           "direction": "south"
//!         }
//!       ],
//!       "stash": {
//!         "items": [],
//!         "item_ids": [
//!           1,
//!           2
//!         ]
//!       },
//!       "room_events": [
//!         1, 4, 2
//!       ],
//!       "narrative": 1,
//!       "subjects": [
//!         1
//!       ]
//!     },
//!     {
//!       "id": 2,
//!       "name": "room 2",
//!       "description": "second room",
//!       "exits": [
//!         {
//!           "room_id": 1,
//!           "direction": "north"
//!         }
//!       ],
//!       "stash": {
//!         "items": [],
//!         "item_ids": []
//!       },
//!       "room_events": [],
//!       "narrative": 2,
//!       "subjects": []
//!     }
//!   ]
//! }"#;
//! let nr = NightRunnerBuilder::new().with_json_data(data).build();
//! let result = nr.parse_input("look");
//! let json_result = nr.json_parse_input("look");
//! assert!(result.is_ok());
//! assert_eq!(result.unwrap(),
//!     ParsingResult::Look(String::from("first room\nHere you see: \n\na item1\na item2"))
//! );
//! assert_eq!(json_result,
//!     "{\"ok\":{\"look\":\"first room\\nHere you see: \\n\\na item1\\na item2\"}}".to_string()
//! );
//! ```
//!
//! for examples of valid YAML and JSON data, see the documentation for
//! the `config` module.
use config::{Config, State};
use parser::interpreter::{EventMessage, ParsingResult};
use std::{cell::RefCell, rc::Rc};
use util::parse_room_text;
pub mod config;
pub mod parser;
pub mod util;

pub type NRResult<T> = Result<T, Box<dyn std::error::Error>>;

/// This is the main struct for this library
/// and represents the game. It holds the state
/// internally and passes it to the parser for
/// processing along with the provided input.
#[derive(Debug, PartialEq)]

pub struct NightRunner {
    state: Rc<RefCell<State>>,
}

/// You can use this struct to build a NightRunner
/// strut. While you can build the NightRunner
/// struct directly, this struct is much more
/// ergonomic.
/// # Examples:
/// ```rust
/// use nightrunner_lib::NightRunnerBuilder;
/// let data = r#"{
///   "allowed_verbs": [
///     {
///       "id": 1,
///       "names": [
///         "quit",
///         ":q",
///         "q"
///       ],
///       "verb_function": "quit"
///     },
///     {
///       "id": 7,
///       "names": [
///         "give",
///         "hand"
///       ],
///       "verb_function": "normal"
///     },
///     {
///       "id": 2,
///       "names": [
///         "help"
///       ],
///       "verb_function": "help"
///     },
///     {
///       "id": 3,
///       "names": [
///         "look",
///         "stare"
///       ],
///       "verb_function": "look"
///     },
///     {
///       "id": 4,
///       "names": [
///         "inventory",
///         "i"
///       ],
///       "verb_function": "inventory"
///     },
///     {
///       "id": 6,
///       "names": [
///         "drop",
///         "place"
///       ],
///       "verb_function": "drop"
///     },
///     {
///       "id": 8,
///       "names": [
///         "talk",
///         "chat"
///       ],
///       "verb_function": "talk"
///     },
///     {
///       "id": 5,
///       "names": [
///         "pick",
///         "take",
///         "grab",
///         "pi",
///         "tk",
///         "gr",
///         "get",
///         "g"
///       ],
///       "verb_function": "take"
///     },
///     {
///       "id": 9,
///       "names": [
///         "hug"
///       ],
///       "verb_function": "normal"
///     }
///   ],
///   "items": [
///     {
///       "id": 1,
///       "name": "item1",
///       "description": "item 1 description",
///       "can_pick": false
///     },
///     {
///       "id": 2,
///       "name": "item2",
///       "description": "item 2 description",
///       "can_pick": true
///     }
///   ],
///   "subjects": [
///     {
///       "id": 1,
///       "name": "subject1",
///       "description": "a subject description",
///       "default_text": "default text"
///     }
///   ],
///   "narratives": [
///     {
///       "id": 1,
///       "text": "text",
///       "description": "text"
///     },
///     {
///       "id": 2,
///       "text": "this is a templated which exists in the game {item1}.\n\nthis is a templated subject that exists in the game {subject1}.",
///       "description": "text"
///     },
///     {
///       "id": 3,
///       "text": "this narrative should replace the old one.",
///       "description": "a replaced narrative"
///     }
///   ],
///   "events": [
///     {
///       "id": 1,
///       "name": "text",
///       "description": "text",
///       "location": 1,
///       "destination": null,
///       "narrative": 1,
///       "required_verb": 2,
///       "required_subject": 1,
///       "required_item": null,
///       "completed": false,
///       "add_item": null,
///       "remove_old_narrative": false,
///       "remove_item": null,
///       "required_events": []
///     },
///     {
///       "id": 2,
///       "name": "text",
///       "description": "text",
///       "location": 1,
///       "destination": null,
///       "narrative": 3,
///       "required_verb": 9,
///       "required_subject": 1,
///       "required_item": null,
///       "completed": false,
///       "add_item": null,
///       "remove_old_narrative": true,
///       "remove_item": null,
///       "required_events": [
///         4
///       ]
///     },
///     {
///       "id": 3,
///       "name": "text",
///       "description": "text",
///       "location": 1,
///       "destination": null,
///       "narrative": 2,
///       "required_verb": 2,
///       "required_subject": 1,
///       "required_item": null,
///       "completed": false,
///       "add_item": null,
///       "remove_old_narrative": true,
///       "remove_item": null,
///       "required_events": [
///         2
///       ]
///     },
///     {
///       "id": 4,
///       "name": "text",
///       "description": "text",
///       "location": 1,
///       "destination": null,
///       "narrative": 1,
///       "required_verb": 8,
///       "required_subject": 1,
///       "required_item": null,
///       "completed": false,
///       "add_item": null,
///       "remove_old_narrative": true,
///       "remove_item": null,
///       "required_events": []
///     }
///   ],
///   "intro": "text",
///   "rooms": [
///     {
///       "id": 1,
///       "name": "room 1",
///       "description": "first room",
///       "exits": [
///         {
///           "room_id": 2,
///           "direction": "south"
///         }
///       ],
///       "stash": {
///         "items": [],
///         "item_ids": [
///           1,
///           2
///         ]
///       },
///       "room_events": [
///         1, 4, 2
///       ],
///       "narrative": 1,
///       "subjects": [
///         1
///       ]
///     },
///     {
///       "id": 2,
///       "name": "room 2",
///       "description": "second room",
///       "exits": [
///         {
///           "room_id": 1,
///           "direction": "north"
///         }
///       ],
///       "stash": {
///         "items": [],
///         "item_ids": []
///       },
///       "room_events": [],
///       "narrative": 2,
///       "subjects": []
///     }
///   ]
/// }"#;
/// let path_to_yaml = "fixtures/";
/// let nr1 = NightRunnerBuilder::new().with_json_data(data);
/// let nr2 = NightRunnerBuilder::new().with_path_for_config(path_to_yaml);
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct NightRunnerBuilder {
    config: Config,
}
impl NightRunnerBuilder {
    /// Creates a new empty NightRunnerBuilder
    /// which contains an empty Config struct.
    pub fn new() -> NightRunnerBuilder {
        NightRunnerBuilder {
            config: Config::default(),
        }
    }
    /// Creates a new NightRunnerBuilder with YAML
    /// data from files in the specified path.
    pub fn with_path_for_config(mut self, path: &str) -> NightRunnerBuilder {
        let config = Config::from_path(path);
        self.config = config;
        self
    }
    /// Creates a new NightRunnerBuilder with JSON
    /// data serialized to a string. This data should
    /// contain the configuration for the whole game.
    pub fn with_json_data(mut self, data: &str) -> NightRunnerBuilder {
        self.config = Config::from_json(data);
        self
    }
    /// Creates a new NightRunner struct. This will fail
    /// if the config is invalid or missing.
    pub fn build(self) -> NightRunner {
        let state = State::init(self.config);
        NightRunner { state }
    }
}

impl NightRunner {
    /// This is the main function that executes the game. Pass
    /// the input string to this function and it will return
    /// a result that can be used on the front-end to display
    /// the game to the user.
    pub fn parse_input(&self, input: &str) -> NRResult<ParsingResult> {
        parser::parse(self.state.clone(), input)
    }
    /// This is the main function that executes the game. Pass
    /// the input string to this function and it will return
    /// a result that can be used on the front-end to display
    /// the game to the user.
    /// Unlike the `parse_input` function, this function will
    /// return the result in JSON format. This is useful for
    /// front-ends that can't integrate with a rust library.
    pub fn json_parse_input(&self, input: &str) -> String {
        let result = parser::parse(self.state.clone(), input);
        let json = match result {
            Ok(ok) => format!("{{\"ok\":{}}}", serde_json::to_string(&ok).unwrap()),
            Err(err) => format!(
                "{{\"error\":{}}}",
                serde_json::to_string(&err.to_string()).unwrap()
            ),
        };
        json
    }
    /// Returns the string with the game intro text. This can
    /// be used to display the game intro to the user, but isn't
    /// required.
    pub fn game_intro(&self) -> String {
        self.state.borrow().config.intro.clone()
    }
    /// Returns the text for the very first room of the game.
    ///
    /// Since there is no input to parse when the game starts,
    /// this function should be used to retrieve that text instead.
    pub fn first_room_text(&self) -> NRResult<EventMessage> {
        let narrative_id = self.state.borrow().rooms[0].narrative.clone();
        let narrative_text = self
            .state
            .borrow()
            .config
            .narratives
            .iter()
            .find(|n| n.id == narrative_id)
            .unwrap()
            .text
            .clone();
        let event_message = parse_room_text(
            self.state.borrow().clone(),
            narrative_text,
            "".to_string(),
            None,
        )?;
        Ok(event_message)
    }
}
