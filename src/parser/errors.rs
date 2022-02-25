//! All possible errors that can occur when parsing
//! the input. These errors should be returned to a
//! front-end for handling display to the user.
//!
//! All errors have Display implemented for them,
//! so they can be easily serialized to a string.

use rand::Rng;
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
impl From<&std::boxed::Box<(dyn std::error::Error + 'static)>> for RequiredEventNotCompleted {
    fn from(_: &std::boxed::Box<(dyn std::error::Error + 'static)>) -> Self {
        RequiredEventNotCompleted
    }
}

/// # Examples
/// ```rust
/// use nightrunner_lib::{NightRunner, NightRunnerBuilder, ParsingResult};
/// use nightrunner_lib::parser::errors::InvalidEvent;
/// use nightrunner_lib::parser::{action::Action};
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
///   "I can't do that.".to_string()
/// );
/// assert_eq!(
///    json_result,
///    r#"{"error":"I can't do that."}"#
/// );
/// ```
#[derive(Debug, Clone)]
pub struct InvalidEvent;
impl std::fmt::Display for InvalidEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rng = rand::thread_rng();
        let error_messages = [
            "Perhaps you should try something else.",
            "Maybe something else needs to be done first.",
            "You can't do that.",
            "I don't understand that.",
            "I don't know how to do that.",
            "I would do anything for love, but I won't do that.",
        ];
        write!(
            f,
            "{}",
            error_messages[rng.gen_range(0..error_messages.len())]
        )
    }
}

impl error::Error for InvalidEvent {}
impl From<&std::boxed::Box<(dyn std::error::Error + 'static)>> for InvalidEvent {
    fn from(_: &std::boxed::Box<(dyn std::error::Error + 'static)>) -> Self {
        InvalidEvent
    }
}

#[derive(Debug, Clone)]
/// Error returned when the action is invalid
pub struct InvalidAction;
impl std::fmt::Display for InvalidAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The action is not valid.")
    }
}

impl error::Error for InvalidAction {}
impl From<&std::boxed::Box<(dyn std::error::Error + 'static)>> for InvalidAction {
    fn from(_: &std::boxed::Box<(dyn std::error::Error + 'static)>) -> Self {
        InvalidAction
    }
}

#[derive(Debug, Clone)]
/// Error returned when the parser tries to access
/// an invalid item. This will likely be an issue
/// in the configuration passed to nightrunner_lib
/// when initializing the parser.
pub struct InvalidItem;
impl std::fmt::Display for InvalidItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The item is invalid.")
    }
}

impl error::Error for InvalidItem {}
impl From<&std::boxed::Box<(dyn std::error::Error + 'static)>> for InvalidItem {
    fn from(_: &std::boxed::Box<(dyn std::error::Error + 'static)>) -> Self {
        InvalidItem
    }
}

#[derive(Debug, Clone)]
/// Error returned when the parser tries to access
/// an invalid subject. This will likely be an issue
/// in the configuration passed to nightrunner_lib
/// when initializing the parser.
pub struct InvalidSubject;
impl std::fmt::Display for InvalidSubject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The subject is invalid.")
    }
}

impl error::Error for InvalidSubject {}
impl From<&std::boxed::Box<(dyn std::error::Error + 'static)>> for InvalidSubject {
    fn from(_: &std::boxed::Box<(dyn std::error::Error + 'static)>) -> Self {
        InvalidSubject
    }
}

#[derive(Debug, Clone)]
/// Error returned when the parser tries to access
/// an invalid verb. This will likely be an issue
/// in the configuration passed to nightrunner_lib
/// when initializing the parser.
pub struct InvalidVerb;
impl std::fmt::Display for InvalidVerb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The verb is invalid.")
    }
}

impl error::Error for InvalidVerb {}
impl From<&std::boxed::Box<(dyn std::error::Error + 'static)>> for InvalidVerb {
    fn from(_: &std::boxed::Box<(dyn std::error::Error + 'static)>) -> Self {
        InvalidVerb
    }
}

#[derive(Debug, Clone)]
/// Error returned when the parser tries to access
/// an invalid movement. This will likely be an issue
/// in the configuration passed to nightrunner_lib
/// when initializing the parser.
pub struct InvalidMovement;
impl std::fmt::Display for InvalidMovement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You can't go that way.")
    }
}

impl error::Error for InvalidMovement {}
impl From<&std::boxed::Box<(dyn std::error::Error + 'static)>> for InvalidMovement {
    fn from(_: &std::boxed::Box<(dyn std::error::Error + 'static)>) -> Self {
        InvalidMovement
    }
}

#[derive(Debug, Clone)]
/// Error returned when the parser tries to access
/// an invalid direction. This will likely be an issue
/// in the configuration passed to nightrunner_lib
/// when initializing the parser.
pub struct InvalidDirection;
impl std::fmt::Display for InvalidDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You can't go that way.")
    }
}

impl error::Error for InvalidDirection {}
impl From<&std::boxed::Box<(dyn std::error::Error + 'static)>> for InvalidDirection {
    fn from(_: &std::boxed::Box<(dyn std::error::Error + 'static)>) -> Self {
        InvalidDirection
    }
}

#[derive(Debug, Clone)]
/// Error returned when the parser tries to access
/// an invalid room. This will likely be an issue
/// in the configuration passed to nightrunner_lib
/// when initializing the parser.
pub struct InvalidRoom;
impl std::fmt::Display for InvalidRoom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The room is invalid.")
    }
}

impl error::Error for InvalidRoom {}
impl From<&std::boxed::Box<(dyn std::error::Error + 'static)>> for InvalidRoom {
    fn from(_: &std::boxed::Box<(dyn std::error::Error + 'static)>) -> Self {
        InvalidRoom
    }
}

#[derive(Debug, Clone)]
/// Error returned when the parser tries to parse
/// an invalid combination of command tokens.
pub struct InvalidVerbItemSubject;
impl std::fmt::Display for InvalidVerbItemSubject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid combination of verb, item, and subject.")
    }
}

impl error::Error for InvalidVerbItemSubject {}
impl From<&std::boxed::Box<(dyn std::error::Error + 'static)>> for InvalidVerbItemSubject {
    fn from(_: &std::boxed::Box<(dyn std::error::Error + 'static)>) -> Self {
        InvalidVerbItemSubject
    }
}

#[derive(Debug, Clone)]
/// Error returned when the parser tries to parse
/// an invalid combination of command tokens.
pub struct InvalidVerbSubject;
impl std::fmt::Display for InvalidVerbSubject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid combination of verb and subject.")
    }
}

impl error::Error for InvalidVerbSubject {}
impl From<&std::boxed::Box<(dyn std::error::Error + 'static)>> for InvalidVerbSubject {
    fn from(_: &std::boxed::Box<(dyn std::error::Error + 'static)>) -> Self {
        InvalidVerbSubject
    }
}

#[derive(Debug, Clone)]
/// Error returned when the parser tries to parse
/// an invalid combination of command tokens.
pub struct InvalidVerbItem;
impl std::fmt::Display for InvalidVerbItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid combination of verb and item.")
    }
}

impl error::Error for InvalidVerbItem {}
impl From<&std::boxed::Box<(dyn std::error::Error + 'static)>> for InvalidVerbItem {
    fn from(_: &std::boxed::Box<(dyn std::error::Error + 'static)>) -> Self {
        InvalidVerbItem
    }
}

#[derive(Debug, Clone)]
/// Error returned when the parser tries to parse
/// a room text and fails.
pub struct ParsingRoomText;
impl std::fmt::Display for ParsingRoomText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There was an error parsing the text for the room.")
    }
}

impl error::Error for ParsingRoomText {}
impl From<&std::boxed::Box<(dyn std::error::Error + 'static)>> for ParsingRoomText {
    fn from(_: &std::boxed::Box<(dyn std::error::Error + 'static)>) -> Self {
        ParsingRoomText
    }
}

#[derive(Debug, Clone)]
/// Error returned when no room is found while
/// parsing the action.
pub struct NoRoom;
impl std::fmt::Display for NoRoom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This room doesn't exist.")
    }
}

impl error::Error for NoRoom {}
impl From<&std::boxed::Box<(dyn std::error::Error + 'static)>> for NoRoom {
    fn from(_: &std::boxed::Box<(dyn std::error::Error + 'static)>) -> Self {
        NoRoom
    }
}

#[derive(Debug, Clone)]
/// Error returned when the player tries to
/// pick up an item marked as cant_pick.
pub struct CantPick;
impl std::fmt::Display for CantPick {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You can't pick that up.")
    }
}

impl error::Error for CantPick {}
impl From<&std::boxed::Box<(dyn std::error::Error + 'static)>> for CantPick {
    fn from(_: &std::boxed::Box<(dyn std::error::Error + 'static)>) -> Self {
        CantPick
    }
}

#[derive(Debug, Clone)]
/// Error returned when trying to remove an item from the player
/// that is not in their inventory.
pub struct NoItem;
impl std::fmt::Display for NoItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You're not carrying that.")
    }
}

impl error::Error for NoItem {}
impl From<&std::boxed::Box<(dyn std::error::Error + 'static)>> for NoItem {
    fn from(_: &std::boxed::Box<(dyn std::error::Error + 'static)>) -> Self {
        NoItem
    }
}

#[derive(Debug, Clone)]
/// Error returned when trying to remove an item from the player
/// that is not in their inventory.
pub struct ItemNotFound;
impl std::fmt::Display for ItemNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The item can't be found.")
    }
}

impl error::Error for ItemNotFound {}
impl From<&std::boxed::Box<(dyn std::error::Error + 'static)>> for ItemNotFound {
    fn from(_: &std::boxed::Box<(dyn std::error::Error + 'static)>) -> Self {
        ItemNotFound
    }
}

#[derive(Debug, Clone)]
/// Error returned when trying to process an empty input.
/// The front-end should handle this scenario, but this
/// error is provided as a convenience.
pub struct EmptyInput;
impl std::fmt::Display for EmptyInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No input. Nothing to process.")
    }
}

impl error::Error for EmptyInput {}
impl From<&std::boxed::Box<(dyn std::error::Error + 'static)>> for EmptyInput {
    fn from(_: &std::boxed::Box<(dyn std::error::Error + 'static)>) -> Self {
        EmptyInput
    }
}

#[derive(Debug, Clone)]
/// Error returned when the parser tries to access
/// an invalid narrative. This will likely be an issue
/// in the configuration passed to nightrunner_lib
/// when initializing the parser.
pub struct InvalidNarrative;
impl std::fmt::Display for InvalidNarrative {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The narrative is invalid.")
    }
}

impl error::Error for InvalidNarrative {}
impl From<&std::boxed::Box<(dyn std::error::Error + 'static)>> for InvalidNarrative {
    fn from(_: &std::boxed::Box<(dyn std::error::Error + 'static)>) -> Self {
        InvalidNarrative
    }
}
