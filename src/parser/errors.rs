//! All possible errors that can occur when parsing
//! the input. These errors should be returned to a
//! front-end for handling display to the user.
//!
//! All errors have Display implemented for them,
//! so they can be easily serialized to a string.

use std::error;
use std::fmt;

/// Event exists but required events haven't been
/// completed yet. The front-end should handle this
/// error state since this isn't really an error,
/// but rather an indication that the action is valid.
///
/// How to handle this depends on what the front-end
/// should do. An example of this state could be
/// and event where you talk to a subject, but you
/// haven't yet completed a previous objective. Talking
/// to the subject would be a valid action, but not
/// currently. Story-wise the subject can be somewhere
/// else, or could return a different narrative instad.
#[derive(Debug, Clone)]
pub struct RequiredEventNotCompleted;
impl std::fmt::Display for RequiredEventNotCompleted {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The required event has not been completed yet.")
    }
}

impl error::Error for RequiredEventNotCompleted {}
impl Into<RequiredEventNotCompleted> for &Box<dyn std::error::Error> {
    fn into(self) -> RequiredEventNotCompleted {
        RequiredEventNotCompleted
    }
}

/// # Examples
/// ```rust
/// use nightrunner_lib::NightRunner;
/// use nightrunner_lib::NightRunnerBuilder;
/// use nightrunner_lib::parser::errors::InvalidEvent;
/// use nightrunner_lib::parser::{action::Action, interpreter::{ParsingResult}};
/// use nightrunner_lib::config::{Verb, VerbFunction, Subject, rooms::Item};
/// let nr = NightRunnerBuilder::new().with_path_for_config("fixtures/").build();
/// let mut result = nr.parse_input("give item2 to subject2");
/// let json_result = nr.json_parse_input("give item2 to subject2");
/// // There is no event for player giving item2 to subject2
/// // so we expect an error. InvalidEvent should be used to
/// // indicate that the event is not valid, and how to handle
/// // this error is up to the front-end. Perhaps you display
/// // a message to the user saying that this action is invalid,
/// // or you don't understand the command.
/// // For convenience, this error wraps the action as it was
/// // interpreted by the parser from the input. This is useful
/// // when writing custom logic for the front-end.
/// assert_eq!(
///     result.unwrap_err().to_string(),
///     InvalidEvent.to_string()
/// );
/// result = nr.parse_input("give item2 to subject2");
/// assert_eq!(
///   result.unwrap_err().to_string(),
///   "The event is not valid.".to_string()
/// );
/// assert_eq!(
///    json_result,
///    r#"{"error":"The event is not valid."}"#
/// );
/// ```
#[derive(Debug, Clone)]
pub struct InvalidEvent;
impl std::fmt::Display for InvalidEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The event is not valid.")
    }
}

impl error::Error for InvalidEvent {}
impl Into<InvalidEvent> for &Box<dyn std::error::Error> {
    fn into(self) -> InvalidEvent {
        InvalidEvent
    }
}

#[derive(Debug, Clone)]
pub struct InvalidAction;
impl std::fmt::Display for InvalidAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The action is not valid.")
    }
}

impl error::Error for InvalidAction {}
impl Into<InvalidAction> for &Box<dyn std::error::Error> {
    fn into(self) -> InvalidAction {
        InvalidAction
    }
}

#[derive(Debug, Clone)]
pub struct InvalidItem;
impl std::fmt::Display for InvalidItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The item is invalid.")
    }
}

impl error::Error for InvalidItem {}
impl Into<InvalidItem> for &Box<dyn std::error::Error> {
    fn into(self) -> InvalidItem {
        InvalidItem
    }
}

#[derive(Debug, Clone)]
pub struct InvalidSubject;
impl std::fmt::Display for InvalidSubject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The subject is invalid.")
    }
}

impl error::Error for InvalidSubject {}
impl Into<InvalidSubject> for &Box<dyn std::error::Error> {
    fn into(self) -> InvalidSubject {
        InvalidSubject
    }
}

#[derive(Debug, Clone)]
pub struct InvalidVerb;
impl std::fmt::Display for InvalidVerb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The verb is invalid.")
    }
}

impl error::Error for InvalidVerb {}
impl Into<InvalidVerb> for &Box<dyn std::error::Error> {
    fn into(self) -> InvalidVerb {
        InvalidVerb
    }
}

#[derive(Debug, Clone)]
pub struct InvalidMovement;
impl std::fmt::Display for InvalidMovement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You can't go that way.")
    }
}

impl error::Error for InvalidMovement {}
impl Into<InvalidMovement> for &Box<dyn std::error::Error> {
    fn into(self) -> InvalidMovement {
        InvalidMovement
    }
}

#[derive(Debug, Clone)]
pub struct InvalidDirection;
impl std::fmt::Display for InvalidDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You can't go that way.")
    }
}

impl error::Error for InvalidDirection {}
impl Into<InvalidDirection> for &Box<dyn std::error::Error> {
    fn into(self) -> InvalidDirection {
        InvalidDirection
    }
}

#[derive(Debug, Clone)]
pub struct InvalidRoom;
impl std::fmt::Display for InvalidRoom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The room is invalid.")
    }
}

impl error::Error for InvalidRoom {}
impl Into<InvalidRoom> for &Box<dyn std::error::Error> {
    fn into(self) -> InvalidRoom {
        InvalidRoom
    }
}

#[derive(Debug, Clone)]
pub struct InvalidVerbItemSubject;
impl std::fmt::Display for InvalidVerbItemSubject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid combination of verb, item, and subject.")
    }
}

impl error::Error for InvalidVerbItemSubject {}
impl Into<InvalidVerbItemSubject> for &Box<dyn std::error::Error> {
    fn into(self) -> InvalidVerbItemSubject {
        InvalidVerbItemSubject
    }
}

#[derive(Debug, Clone)]
pub struct InvalidVerbSubject;
impl std::fmt::Display for InvalidVerbSubject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid combination of verb and subject.")
    }
}

impl error::Error for InvalidVerbSubject {}
impl Into<InvalidVerbSubject> for &Box<dyn std::error::Error> {
    fn into(self) -> InvalidVerbSubject {
        InvalidVerbSubject
    }
}

#[derive(Debug, Clone)]
pub struct InvalidVerbItem;
impl std::fmt::Display for InvalidVerbItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid combination of verb and item.")
    }
}

impl error::Error for InvalidVerbItem {}
impl Into<InvalidVerbItem> for &Box<dyn std::error::Error> {
    fn into(self) -> InvalidVerbItem {
        InvalidVerbItem
    }
}

#[derive(Debug, Clone)]
pub struct ParsingRoomText;
impl std::fmt::Display for ParsingRoomText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There was an error parsing the text for the room.")
    }
}

impl error::Error for ParsingRoomText {}
impl Into<ParsingRoomText> for &Box<dyn std::error::Error> {
    fn into(self) -> ParsingRoomText {
        ParsingRoomText
    }
}

#[derive(Debug, Clone)]
pub struct NoRoom;
impl std::fmt::Display for NoRoom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This room doesn't exist.")
    }
}

impl error::Error for NoRoom {}
impl Into<NoRoom> for &Box<dyn std::error::Error> {
    fn into(self) -> NoRoom {
        NoRoom
    }
}

#[derive(Debug, Clone)]
pub struct CantPick;
impl std::fmt::Display for CantPick {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You can't pick that up.")
    }
}

impl error::Error for CantPick {}
impl Into<CantPick> for &Box<dyn std::error::Error> {
    fn into(self) -> CantPick {
        CantPick
    }
}

#[derive(Debug, Clone)]
pub struct NoItem;
impl std::fmt::Display for NoItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You're not carrying that.")
    }
}

impl error::Error for NoItem {}
impl Into<NoItem> for &Box<dyn std::error::Error> {
    fn into(self) -> NoItem {
        NoItem
    }
}

#[derive(Debug, Clone)]
pub struct ItemNotFound;
impl std::fmt::Display for ItemNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The item can't be found.")
    }
}

impl error::Error for ItemNotFound {}
impl Into<ItemNotFound> for &Box<dyn std::error::Error> {
    fn into(self) -> ItemNotFound {
        ItemNotFound
    }
}

#[derive(Debug, Clone)]
pub struct EmptyInput;
impl std::fmt::Display for EmptyInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No input. Nothing to process.")
    }
}

impl error::Error for EmptyInput {}
impl Into<EmptyInput> for &Box<dyn std::error::Error> {
    fn into(self) -> EmptyInput {
        EmptyInput
    }
}

#[derive(Debug, Clone)]
pub struct InvalidNarrative;
impl std::fmt::Display for InvalidNarrative {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The narrative is invalid.")
    }
}

impl error::Error for InvalidNarrative {}
impl Into<InvalidNarrative> for &Box<dyn std::error::Error> {
    fn into(self) -> InvalidNarrative {
        InvalidNarrative
    }
}
