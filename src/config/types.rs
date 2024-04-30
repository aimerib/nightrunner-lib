use self::directions::Directions;

use super::*;
use tsify::Tsify;

/// This struct holds the texts used to display the story
/// in the game. These narratives are used to display
/// texts for events as well as the current text in the room.
///
/// Some of the text displayed in the game comes from the
/// item, room, or subject's description. For everything
/// else, the narrative's text is used.
#[derive(Tsify, Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "snake_case")]
pub struct Narrative {
    /// Narrative id used when referencing the narrative.
    pub id: u16,
    /// The actual text of the narrative to be displayed.
    pub text: String,
    /// This is a human readable name for the narrative.
    pub description: String,
}

/// Verbs can be named anything, but a few are reserved for specific functions.
/// The verb_function field is used to determine what function the verb serves.
///
/// The following verbs functions need to be declared:
/// - look
/// - inventory
/// - help
/// - quit
/// - take
/// - drop
/// - talk
///
/// for all other verbs you can use the VerbFunction::Normal.
/// VerbFunction::Normal is the default, and will be parsed by
/// the events parser.
///
/// Verbs can have multiple names, but only one function. If
/// a verb has more than one name you can use any of the names
/// available to trigger the verb.
///
/// For example, if you have a verb that is named 'look' and
/// 'peek' you can use either of the two names to trigger
/// the look action.
///
/// # Examples
/// ```rust
/// # use nightrunner_lib::config::{Verb, VerbFunction};
/// let verb1 = Verb {
///    id: 1,
///    names: vec!["look".to_string(), "peek".to_string()],
///    verb_function: VerbFunction::Look,
/// };
///
/// let verb2 = Verb {
///    id: 2,
///    names: vec!["take".to_string(), "pick".to_string()],
///    verb_function: VerbFunction::Take,
/// };
///
/// let verb3 = Verb {
///    id: 3,
///    names: vec!["parkour".to_string(), "flip".to_string()],
///    verb_function: VerbFunction::Normal,
/// };
/// ```
#[derive(Tsify, Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "snake_case")]
pub struct Verb {
    /// The id of the verb used when referencing the verb.
    pub id: u16,
    /// A verb can be named anything and can have multiple aliases,
    /// so commands like `look` and `peek` can be used interchangeably.
    pub names: Vec<String>,
    /// The function that the verb serves. Since some verbs are reserved
    /// for specific functions, this field is used to determine what
    /// function the verb serves, and this allows verbs to be named
    /// anything. For the possible functions see the [VerbFunction] enum.
    pub verb_function: VerbFunction,
}

/// The VerbFunction enum is used to determine what function
/// the verb serves. Some verbs are reserved for specific
/// functions but can named anything. For example, the verb
/// 'look' is used to look at the room or item but it can
/// be named anything.
#[derive(Tsify, Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "snake_case")]
pub enum VerbFunction {
    #[serde(rename = "quit")]
    /// The quit verb is used to quit the game.
    Quit,
    #[serde(rename = "help")]
    /// The help verb is used to display the help text.
    Help,
    #[serde(rename = "look")]
    /// The look verb is used to look at a room, item, or subject.
    Look,
    #[serde(rename = "inventory")]
    /// The inventory verb is used to display the inventory.
    Inventory,
    #[serde(rename = "take")]
    /// The take verb is used to take an item from a room.
    /// Some items can't be picked up, and some other items
    /// can only be given to the player through an event.
    Take,
    #[serde(rename = "drop")]
    /// The drop verb is used to drop an item in a room. When
    /// a player drops an item, the item is removed from the
    /// player's inventory and placed in the room where it can
    /// be retrieved again.
    Drop,
    #[serde(rename = "talk")]
    /// The talk verb is used to talk to a character in a room.
    Talk,
    #[serde(rename = "normal")]
    /// Any other verbs should be set to this variant and will
    /// be parsed by the event handling function.
    Normal,
}

/// Subjects are the people or things that can be interacted with.
///
/// For example, a person can be a subject, but an item cannot.
///
/// Some examples of things that can be subjects are:
/// - A person
/// - An object such as a desk, a chair, or a computer
/// - An animal
/// - A door
///
/// Example:
/// ```rust
/// # use nightrunner_lib::config::Subject;
/// let subject = Subject {
///    id: 1,
///    name: "person".to_string(),
///    description: "A person dressed all in black".to_string(),
///    default_text: "Person: I'm busy now. Maybe later.".to_string(),
/// };
/// ```
#[derive(Tsify, Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "snake_case")]
pub struct Subject {
    /// The id of the subject used when referencing the subject.
    pub id: u16,
    /// The name of the subject.
    pub name: String,
    /// This is what the parser will use when the player
    /// looks at the subject.
    pub description: String,
    /// The default text to display when the the player
    /// interacts with the subject and no active events
    /// are associated with this subject.
    pub default_text: String,
}

/// An event controls the flow of the game.
/// You can have multiple events in a room.
///
/// Some events require other events to be completed first.
/// Events are triggered by verbs.
///
/// Once an event is marked as completed, it will not be
/// triggered again.
///
/// # Examples:
///
/// An event that happens in room 1 as a response to
/// talking to subject 2:
/// ```rust
/// # use nightrunner_lib::config::Event;
/// let event = Event {
///   id: 1,
///   location: 1,
///   name: "Talking to subject 2".to_string(),
///   description: "This event happens when you talk to subject 2.".to_string(),
///   destination: None,
///   narrative: Some(2),
///   required_verb: Some(3),
///   required_subject: Some(2),
///   required_item: None,
///   completed: false,
///   add_item: None,
///   remove_old_narrative: false,
///   remove_item: None,
///   required_events: Vec::new(),
///   add_subject: None,
///   move_subject_to_location: None,
///   narrative_after: None,
///   remove_subject: false,
/// };
/// ```
///
/// An event that happens in room 2 as a response to
/// using an item with a subject and requires event
/// 1 to be completed:
/// ```rust
/// # use nightrunner_lib::config::Event;
/// let event = Event {
///   id: 2,
///   location: 2,
///   name: "Using item 3 with subject 1".to_string(),
///   description: "This event happens when you use an item with subject 1.".to_string(),
///   destination: None,
///   narrative: Some(4),
///   // here verb id 3 would be marked with VerbFunction::Normal
///   required_verb: Some(3),
///   required_subject: Some(1),
///   required_item: Some(3),
///   completed: false,
///   add_item: None,
///   remove_old_narrative: false,
///   // here item id 3 would be removed after the event is completed
///   remove_item: Some(3),
///   required_events: Vec::new(),
///   add_subject: None,
///   move_subject_to_location: None,
///   narrative_after: None,
///   remove_subject: false,
/// };
/// ```

#[derive(Tsify, Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "snake_case")]
pub struct Event {
    /// The id of the event used when referencing the event.
    pub id: u16,
    /// Room id where the event happens.
    pub location: u16,
    /// Name of the event. This isn't used in the game
    /// and is used to make the event configuration
    /// more readable.
    pub name: String,
    /// This is the description of the event. Also
    /// not used in the game, but rather as a way
    /// to make the event configuration more readable.
    pub description: String,
    /// If the event takes you to a different room,
    /// this is the room id.
    pub destination: Option<u16>,
    /// Narrative id to be displayed when the event
    /// is triggered.
    pub narrative: Option<u16>,
    /// Verb id that triggers the event.
    pub required_verb: Option<u16>,
    /// Subject id that triggers the event.
    pub required_subject: Option<u16>,
    /// Item id that triggers the event.
    pub required_item: Option<u16>,
    /// If the event is completed, it won't be triggered again.
    pub completed: bool,
    /// If the event adds an item to the inventory,
    /// this is the item id.
    pub add_item: Option<u16>,
    /// If the room narrative should be different after
    /// this event, this should be true. Default is false.
    #[serde(default)]
    pub remove_old_narrative: bool,
    /// If a new narrative should be displayed after this
    /// event, this should be the id of the new narrative.
    pub narrative_after: Option<u16>,
    /// If the event removes an item from the inventory,
    /// this is the item id.
    pub remove_item: Option<u16>,
    /// If the event requires other events to be completed,
    /// this is a list of event ids that need to be completed
    /// before this event can be triggered.
    pub required_events: Vec<u16>,
    /// If the event brings a new subject to the room, this is
    /// the subject id.
    pub add_subject: Option<u16>,
    /// If the event removes a subject from the room, this needs
    /// to be true.
    #[serde(default)]
    pub remove_subject: bool,
    /// If in addition to removing the subject from the room,
    /// the event also moves the subject to a different room,
    /// this is the new room id.
    pub move_subject_to_location: Option<u16>,
}

/// This struct represents an item in the game.
/// It contains the name of the item, the description
/// and whether or not the item can be picked up.
#[derive(Tsify, Debug, Clone, PartialEq, Deserialize, Serialize, Eq, PartialOrd, Ord)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "snake_case")]
pub struct Item {
    /// The id of the item used when referencing the item.
    pub id: u16,
    /// The name of the item.
    pub name: String,
    /// The description of the item.
    /// This is used when the player looks at
    /// the item.
    pub description: String,
    /// Whether or not the item can be picked up.
    /// If this is true then the item can be
    /// picked up by the player. Most of the times
    /// if an item can't be picked up you will
    /// want to use a subject instead.
    pub can_pick: bool,
}

/// This struct represents the storage for both the player
/// and the room and implements functions to add and remove
/// items from the storage.
#[derive(Tsify, Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Deserialize, Serialize, Default)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "snake_case")]
pub struct Storage {
    /// This field contains the list of actual
    /// items available in the storage struct
    /// and gets populated during the state
    /// initialization based on the item_ids field
    pub items: Vec<Item>,
    // /// The list of item ids that are currently
    // /// available in storage. Only used for the
    // /// configuration data.
}

/// This struct represents the player's current state.
#[derive(Tsify, Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Player {
    /// The player's inventory
    pub inventory: Storage,
}

/// This struct represents exits from a room.
#[derive(Tsify, Debug, Clone, Deserialize, Serialize, Eq, Ord, PartialEq, PartialOrd)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "snake_case")]
pub struct Exit {
    /// The room that the exit leads to.
    pub room_id: u16,
    /// The direction this direction is located.
    pub direction: Directions,
}

/// This struct represents a room in the game.
#[derive(Tsify, Debug, Clone, Deserialize, Serialize, Eq, Ord, PartialEq, PartialOrd)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "snake_case")]
pub struct Room {
    /// The unique identifier for this room.
    pub id: u16,
    /// The name of the room. This is used only
    /// for making reading the game configs more
    /// human readable.
    pub name: String,
    /// This is a description of the room used
    /// when looking at the room or for the
    /// description shown in the exists list of
    /// the parsed result.
    pub description: String,
    /// This is the list of possible exits from
    /// this room. If the player tries to move
    /// in a direction that is not in this list
    /// then the player will be told that they
    /// can't go that way.
    pub exits: Vec<Exit>,
    /// This is the list of items that are
    /// currently in the room.
    pub stash: Storage,
    /// This is the list of events that can
    /// be triggered in this room.
    pub events: Vec<Event>,
    /// This is the actual text displayed
    /// when the user enters a room.
    /// If an event completed in this room
    /// doesn't replace this narrative, the
    /// same narrative will be shown every time
    /// the player enters this room, otherwise
    /// the new narrative will be displayed instead.
    pub narrative: u16,
    /// This is the list of subjects that can
    /// be interacted with in this room.
    pub subjects: Vec<Subject>,
}

/// This struct represents a room blueprint.
/// This is used to create the actual rooms
/// that the player will interact with.
#[derive(Tsify, Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct RoomBlueprint {
    pub(crate) id: u16,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) exits: Vec<Exit>,
    pub(crate) item_ids: Vec<u16>,
    pub(crate) narrative: u16,
    pub(crate) subject_ids: Vec<u16>,
}
