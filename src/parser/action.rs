use crate::config::directions::Directions;
use crate::config::rooms::Item;
use crate::config::{State, Subject, Verb};
use regex::Regex;
use serde::{Deserialize, Serialize};

/// Describes the type of action that is
/// being parsed.
/// They are determined based on the
/// combination of input tokens. If
/// an action contains only a verb,
/// such as "look", then it is a
/// `ActionType::Verb` and so on.
/// Invalid actions are returned
/// when the action parser can't
/// determine the type of action.
#[derive(Debug, PartialEq)]
pub enum ActionType {
    Verb,
    VerbSubject,
    VerbItem,
    VerbItemSubject,
    Invalid,
    Movement,
}

/// Actions are the core of the parser.
/// They are used to determine what the
/// player is trying to do.
///
/// The struct `Action` implements its own
/// parser and return a valid action struct
/// with the appropriate data.
///
/// # Examples:
///
/// * "look subject1"
/// ```ignore
/// Action {
///     verb: Some(Verb {
///         id: 1,
///         names: ["look"],
///         verb_function: VerbFunction::Look
///     }),
///     subject: Some(Subject {
///         id: 1,
///         name: "subject1",
///         description: "some verb text"
///     }),
///     item: None,
///     movement: None,
/// }
/// ```
///
/// * "south"
/// ```ignore
/// Action {
///     verb: None,
///     subject: None,
///     item: None,
///     movement: Some(Directions::South),
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Action {
    /// If the action contains a verb,
    /// then this field will be set with the
    /// verb. Currently every action has to
    /// have a verb.
    pub verb: Option<Verb>,
    /// If the action contains a subject,
    /// then this field will be set with the
    /// subject.
    pub subject: Option<Subject>,
    /// If the action contains an item,
    /// then this field will be set with the
    /// item.
    pub item: Option<Item>,
    /// If the action contains a direction,
    /// then this field will be set with the
    /// direction and all other fields will
    /// be set to None.
    pub movement: Option<Directions>,
    pub command_tokens: Vec<String>,
    pub input: String,
}

impl Action {
    /// Is the action valid?
    pub fn is_valid(&self) -> bool {
        if self.verb.is_some() {
            self.item.is_some()
                || self.subject.is_some()
                || self.movement.is_some()
                || self.verb.is_some()
        } else {
            self.movement.is_some()
        }
    }
    /// Determines the type of action.
    pub fn action_type(&self) -> ActionType {
        if self.is_valid() && self.verb.is_some() && self.item.is_some() && self.subject.is_some() {
            ActionType::VerbItemSubject
        } else if self.is_valid() && self.verb.is_some() && self.subject.is_some() {
            ActionType::VerbSubject
        } else if self.is_valid() && self.verb.is_some() && self.item.is_some() {
            ActionType::VerbItem
        } else if self.is_valid() && self.verb.is_some() {
            ActionType::Verb
        } else if self.is_valid() && self.movement.is_some() {
            ActionType::Movement
        } else {
            ActionType::Invalid
        }
    }
    /// Parses the action.
    ///
    /// It will return an action struct with the
    /// appropriate data.
    /// This function tokenizes the input string
    /// and drops any words contained in the prepositions
    /// or determiners arrays.
    /// If after filtering the input string nothing is left,
    /// it returns an invalid action with all fields set to None.
    pub fn parse(state: &State, input: &str) -> Action {
        let prepositions = state.config.allowed_prepositions.clone().prepositions;
        let determiners = state.config.allowed_determiners.clone().determiners;

        let command_tokens: Vec<String> = input
            .split(' ')
            .collect::<Vec<&str>>()
            .iter()
            .filter(|w| {
                let word: String = w.to_string().to_lowercase();
                !prepositions.contains(&word) && !determiners.contains(&word)
            })
            .map(|word| word.to_string())
            .collect::<Vec<String>>();
        if command_tokens.is_empty() {
            Action {
                item: None,
                movement: None,
                subject: None,
                verb: None,
                command_tokens: vec!["".to_string()],
                input: input.to_string(),
            }
        } else {
            parse_action(state, command_tokens, input)
        }
    }
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.is_valid() {
            true => {
                if self.verb.is_some() && self.item.is_some() && self.subject.is_some() {
                    write!(
                        f,
                        "{} {} {}",
                        self.verb.clone().unwrap(),
                        self.item.clone().unwrap(),
                        self.subject.clone().unwrap().name
                    )
                } else if self.verb.is_some() && self.subject.is_some() {
                    write!(
                        f,
                        "{} {}",
                        self.verb.clone().unwrap(),
                        self.subject.clone().unwrap().name
                    )
                } else if self.verb.is_some() && self.item.is_some() {
                    write!(
                        f,
                        "{} {}",
                        self.verb.clone().unwrap(),
                        self.item.clone().unwrap()
                    )
                } else if self.verb.is_some() {
                    write!(f, "{}", self.verb.clone().unwrap())
                } else {
                    write!(f, "{}", self.movement.clone().unwrap())
                }
            }
            false => write!(f, "Invalid action: {}", self.command_tokens.join(" ")),
        }
    }
}

impl From<Action> for String {
    fn from(action: Action) -> String {
        action.to_string()
    }
}

impl From<&Action> for String {
    fn from(action: &Action) -> String {
        action.to_string()
    }
}

fn parse_action(state: &State, command_tokens: Vec<String>, input: &str) -> Action {
    let verb = extract_verb(state, &command_tokens);
    let movement = extract_movement(state, &command_tokens);
    let subject = extract_subject(state, &command_tokens);
    let item = extract_item(state, &command_tokens, input);
    Action {
        verb,
        movement,
        item,
        subject,
        command_tokens,
        input: input.to_string(),
    }
}

fn extract_verb(state: &State, command_tokens: &Vec<String>) -> Option<Verb> {
    let verbs = state.config.allowed_verbs.clone();
    if let Some(verb) = verbs.iter().find(|v| v.names.contains(&command_tokens[0])) {
        Some(verb.clone())
    } else {
        None
    }
}

fn extract_item(state: &State, command_tokens: &Vec<String>, input: &str) -> Option<Item> {
    let subjects = state.config.subjects.clone();
    let items_string: String = state
        .config
        .items
        .iter()
        .map(|item| &item.name[..])
        .collect::<Vec<&str>>()
        .join("|");
    let items_regex_match = format!("({})", items_string);
    let re = Regex::new(&items_regex_match[..]).unwrap();

    let item = if command_tokens.len() > 1
        && subjects
            .iter()
            .find(|s| &s.name == &command_tokens[1])
            .is_none()
    {
        if let Some(capture) = re.captures(&input) {
            match capture.get(1) {
                Some(_) => {
                    let item = state
                        .config
                        .items
                        .iter()
                        .find(|item| item.name == capture.get(1).unwrap().as_str().to_string())
                        .unwrap()
                        .to_owned();
                    Some(item.to_owned())
                }
                None => None,
            }
        } else {
            None
        }
    } else {
        None
    };
    item
}

fn extract_subject(state: &State, command_tokens: &[String]) -> Option<Subject> {
    let subjects = state.config.subjects.clone();
    match &command_tokens.len() {
        0 | 1 => None,
        2 => {
            if let Some(subject) = subjects.iter().find(|s| &s.name == &command_tokens[1]) {
                Some(subject.clone())
            } else {
                None
            }
        }
        _ => {
            if let Some(subject) = subjects
                .iter()
                .find(|s| &s.name == &command_tokens[&command_tokens.len() - 1])
            {
                Some(subject.clone())
            } else {
                None
            }
        }
    }
}

// This should be re-worked to use events instead. Maybe v2.0
// Using events allows for commands such "sneak north" to get
// past a sleeping dragon, or a corporate goon standing guard.
// As it stands, the parser is very simple when it comes to mo-
// ving around.
fn extract_movement(state: &State, command_tokens: &[String]) -> Option<Directions> {
    let movements = state.config.allowed_movements.movements.clone();
    let directions = state.config.allowed_directions.directions.clone();
    match command_tokens.len() {
        1 => match &command_tokens[0][..] {
            "north" | "n" => Some(Directions::NORTH),
            "south" | "s" => Some(Directions::SOUTH),
            "east" | "e" => Some(Directions::EAST),
            "west" | "w" => Some(Directions::WEST),
            _ => None,
        },
        2 => {
            if movements.contains(&command_tokens[0]) && directions.contains(&command_tokens[1]) {
                match &command_tokens[1][..] {
                    "north" | "n" => Some(Directions::NORTH),
                    "south" | "s" => Some(Directions::SOUTH),
                    "east" | "e" => Some(Directions::EAST),
                    "west" | "w" => Some(Directions::WEST),
                    _ => None,
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

#[cfg(test)]
#[path = "action_tests.rs"]
mod action_tests;
