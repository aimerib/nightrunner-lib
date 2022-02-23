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
//! use nightrunner_lib::util::test_helpers::mock_json_data;
//! let data = mock_json_data();
//! let nr = NightRunnerBuilder::new().with_json_data(data).build();
//! let result = nr.parse_input("look");
//! let json_result = nr.json_parse_input("look");
//! assert!(result.is_ok());
//! assert_eq!(result.unwrap(),
//!     ParsingResult::Look(String::from("first room\n\nHere you see: \nan item1\nan item2\nsubject1"))
//! );
//! assert_eq!(json_result,
//!     "{\"ok\":{\"look\":\"first room\\n\\nHere you see: \\nan item1\\nan item2\\nsubject1\"}}".to_string()
//! );
//! ```
//!
//! for examples of valid YAML and JSON data, see the documentation for
//! the `config` module.
#![warn(missing_docs)]
use config::{Config, State};
use parser::interpreter::EventMessage;
use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, error::Error, rc::Rc};
use util::parse_room_text;
/// Module containing the configuration code for this
/// library.
pub mod config;
/// The parser module contains a single function that
/// parses the input string and returns a `ParsingResult`.
pub mod parser;
/// Helper functions.
pub mod util;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

/// We use a type alias to make error handling easier.
pub type NRResult<T> = Result<T, Box<dyn Error>>;

/// This is the result of the parsing of the input.
/// Each variant contains the output for the game and
/// should be used by a front-end to display to the user.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ParsingResult {
    /// Returned when the player sends a command corresponding
    /// to a verb that has VerbFunction::Help as its verb_function.
    /// The value is a string generated by the library displaying
    /// general commands as well as verbs available in the game.
    Help(String),
    /// Returned when the player sends a command for looking at the
    /// room, an item, or a subject. The value will be the message
    /// returned from the parser with the description associated with
    /// the room, item, or subject.
    Look(String),
    /// Returned when the player receives a new item, either by picking
    /// it up or by receiving it as the result of an event.
    NewItem(String),
    /// Returned when the player loses an item, either by dropping it
    /// or by losing it as the result of an event.
    DropItem(String),
    /// Returned when the player issues a command with a verb that
    /// has VerbFunction::Inventory as its verb_function. The value is
    /// a string containing each item in the player's inventory.
    Inventory(String),
    /// Returned when the player issues a command that interacts with
    /// a subject without a current event associated with it. The value
    /// is the default text for the subject.
    SubjectNoEvent(String),
    /// Returned when an event is triggered by the player's command. The
    /// returned struct contains the text to be displayed to the player.
    EventSuccess(EventMessage),
    /// Returned when the player issues a command with a verb that has
    /// VerbFunction::Quit as its verb_function. This variant is used
    /// to indicate to the front-end that the game should be quit.
    /// Implementation of how to quit the game is left to the front-end.
    Quit,
}

/// This is the main struct for this library
/// and represents the game. It holds the state
/// internally and passes it to the parser for
/// processing along with the provided input.
#[wasm_bindgen]
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
/// use nighrunner_lib::util::test_helpers::mock_json_data;
/// let data = mock_json_data();
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

#[cfg(not(target_arch = "wasm32"))]
/// # Nightrunner Rust Library
///
/// Use this implementation when using this library
/// in a Rust application.
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

#[cfg(any(target_arch = "wasm32", doc))]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "messageType", content = "data")]
#[serde(rename_all = "camelCase")]
/// When compiling for the web, this struct is used to
/// serialize the game state to JSON. `messageType` is
/// the type of the message returned by the library to
/// indicate which action was processed by the parser.
pub enum JsMessage {
    /// Returned when the player sends a command corresponding
    /// to a verb that has VerbFunction::Help as its verb_function.
    /// The value is a string generated by the library displaying
    /// general commands as well as verbs available in the game.
    Help(String),
    /// Returned when the player sends a command for looking at the
    /// room, an item, or a subject. The value will be the message
    /// returned from the parser with the description associated with
    /// the room, item, or subject.
    Look(String),
    /// Returned when the player receives a new item, either by picking
    /// it up or by receiving it as the result of an event.
    NewItem(String),
    /// Returned when the player loses an item, either by dropping it
    /// or by losing it as the result of an event.
    DropItem(String),
    /// Returned when the player issues a command with a verb that
    /// has VerbFunction::Inventory as its verb_function. The value is
    /// a string containing each item in the player's inventory.
    Inventory(String),
    /// Returned when the player issues a command that interacts with
    /// a subject without a current event associated with it. The value
    /// is the default text for the subject.
    SubjectNoEvent(String),
    /// Returned when an event is triggered by the player's command. The
    /// returned struct contains the text to be displayed to the player.
    EventSuccess(EventMessage),
}

#[cfg(any(target_arch = "wasm32", doc))]
#[wasm_bindgen]
/// # Nightrunner Wasm Library
///
/// Use this implementation when using this library
/// in a WebAssembly browser application. This is the
/// implementation exposed when compiling with `--target=wasm32-unknown-unknown`.
impl NightRunner {
    /// When using the wasm library we won't have access to the
    /// builder patter, so the constructor needs to receive the
    /// configuration for games as a parameter.
    ///
    /// config should be a JSON string.
    #[wasm_bindgen(constructor)]
    pub fn new(config: &str) -> NightRunner {
        let config = Config::from_json(config);
        let state = State::init(config);
        NightRunner { state }
    }
    /// This is the main function that executes the game. Pass
    /// the input string to this function and it will return
    /// a result that can be used on the front-end to display
    /// the game to the user.
    /// Unlike the non-wasm version, this function will return
    /// the result in JSON format. The conversion of the result
    /// to JSON is done by the `JsValue::from_serde` function from
    /// wasm_bindgen.
    pub fn parse(&self, input: &str) -> Result<JsValue, JsError> {
        let result = parser::parse(self.state.clone(), input);
        match result {
            Ok(ok) => {
                let message = match ok {
                    ParsingResult::Look(msg) => JsMessage::Look(msg),
                    ParsingResult::Help(msg) => JsMessage::Help(msg),
                    ParsingResult::NewItem(msg) => JsMessage::NewItem(msg),
                    ParsingResult::DropItem(msg) => JsMessage::DropItem(msg),
                    ParsingResult::Inventory(msg) => JsMessage::Inventory(msg),
                    ParsingResult::SubjectNoEvent(msg) => JsMessage::SubjectNoEvent(msg),
                    ParsingResult::EventSuccess(event_msg) => JsMessage::EventSuccess(event_msg),
                };
                Ok(JsValue::from_serde(&message).unwrap())
            }
            Err(err) => Err(JsError::new(&err.to_string())),
        }
    }

    /// Returns the string with the game intro text. This can
    /// be used to display the game intro to the user, but isn't
    /// required.
    #[wasm_bindgen]
    pub fn game_intro(&self) -> String {
        self.state.borrow().config.intro.clone()
    }
    /// Returns the text for the very first room of the game.
    ///
    /// Since there is no input to parse when the game starts,
    /// this function should be used to retrieve that text instead.
    pub fn first_room_text(&self) -> Result<JsValue, JsError> {
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
        )
        .unwrap();
        Ok(JsValue::from_serde(&event_message).unwrap())
    }
}
