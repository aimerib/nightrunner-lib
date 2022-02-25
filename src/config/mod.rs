pub(crate) mod determiners;
pub(crate) mod directions;
pub(crate) mod movements;
pub(crate) mod prepositions;
pub(crate) mod rooms;

use crate::parser::errors::{InvalidRoom, InvalidSubject, NoItem};
use crate::NRResult;

use self::determiners::AllowedDeterminers;
use self::directions::AllowedDirections;
use self::movements::AllowedMovements;
use self::prepositions::AllowedPrepositions;
use self::rooms::{Room, RoomBlueprint};
use serde::{Deserialize, Serialize};
use serde_json;

/// This struct holds the texts used to display the story
/// in the game. These narratives are used to display
/// texts for events as well as the current text in the room.
///
/// Some of the text displayed in the game comes from the
/// item, room, or subject's description. For everything
/// else, the narrative's text is used.
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
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
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
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
impl std::fmt::Display for Verb {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.verb_function {
            VerbFunction::Look => write!(f, "look"),
            VerbFunction::Help => write!(f, "help"),
            VerbFunction::Take => write!(f, "take"),
            VerbFunction::Drop => write!(f, "drop"),
            VerbFunction::Inventory => write!(f, "inventory"),
            VerbFunction::Quit => write!(f, "quit"),
            VerbFunction::Talk => write!(f, "talk"),
            VerbFunction::Normal => write!(f, "{}", self.names[0]),
        }
    }
}

/// The VerbFunction enum is used to determine what function
/// the verb serves. Some verbs are reserved for specific
/// functions but can named anything. For example, the verb
/// 'look' is used to look at the room or item but it can
/// be named anything.
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
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
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
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

impl std::fmt::Display for Subject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.name[..])
    }
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

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
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

impl Event {
    /// Checks if a task is completed.
    ///
    /// This function returns true if the task is completed, and false otherwise.
    pub fn is_completed(&self) -> bool {
        self.completed
    }
    /// Marks an event as completed.
    ///
    /// This function sets the `completed` field of the event to `true`.
    pub fn complete(&mut self) {
        self.completed = true;
    }
}

/// This struct represents an item in the game.
/// It contains the name of the item, the description
/// and whether or not the item can be picked up.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Eq, PartialOrd, Ord)]
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

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.name[..])
    }
}

/// This struct holds the data deserialized from JSON.
/// This is a temporary holding place for deserialing this data
/// since the Config struct has fields that are loaded from data
/// compiled with the library. It would make it so these fields
/// have to be sent with the JSON data to deserialize a Config
/// struct. Instead we deserialize the data into this struct and
/// then we can just copy the fields into the Config struct.
#[derive(Deserialize, Debug)]
struct ConfigData {
    items: Vec<Item>,
    narratives: Vec<Narrative>,
    room_blueprints: Vec<RoomBlueprint>,
    subjects: Vec<Subject>,
    events: Vec<Event>,
    intro: String,
    allowed_verbs: Vec<Verb>,
}

/// This holds the configurations for the game.
/// This includes the possible verbs, directions,
/// items, subjects, narratives, events, intro,
/// and rooms.
///
/// These configurations can be loaded from YAML
/// files, or from serialized JSON data.
///
/// You should choose YAML files when using this
/// library with a terminal console or local
/// front-end. Otherwise you should use JSON for
/// most use cases.
///
/// While you can create this struct manually,
/// you should use the Config::init_yaml or the
/// Config::init_json functions to load the data
/// from a YAML file or JSON data.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Eq)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    /// This field is hardcoded in the library.
    pub allowed_prepositions: AllowedPrepositions,
    /// This field is hardcoded in the library.
    pub allowed_determiners: AllowedDeterminers,
    /// This field is hardcoded in the library.
    pub allowed_movements: AllowedMovements,
    /// This field is hardcoded in the library.
    pub allowed_directions: AllowedDirections,
    /// All the allowed verbs in the game.
    pub allowed_verbs: Vec<Verb>,
    /// All the possible items in the game.
    pub items: Vec<Item>,
    /// All the possible subjects in the game.
    pub subjects: Vec<Subject>,
    /// All the possible narratives in the game.
    pub narratives: Vec<Narrative>,
    /// All the possible events in the game.
    pub events: Vec<Event>,
    /// The intro text to be displayed when the game starts.
    pub intro: String,
    pub(crate) room_blueprints: Vec<RoomBlueprint>,
    // /// All the possible rooms in the game.
    // pub rooms: Vec<Room>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            allowed_verbs: Vec::new(),
            allowed_prepositions: AllowedPrepositions {
                prepositions: Vec::new(),
            },
            allowed_determiners: AllowedDeterminers {
                determiners: Vec::new(),
            },
            allowed_movements: AllowedMovements {
                movements: Vec::new(),
            },
            allowed_directions: AllowedDirections {
                directions: Vec::new(),
            },
            items: Vec::new(),
            subjects: Vec::new(),
            narratives: Vec::new(),
            room_blueprints: Vec::new(),
            events: Vec::new(),
            intro: String::new(),
        }
    }
}

impl Config {
    /// # Config::init_yaml
    /// Loads config from serialized JSON.
    ///
    /// This is useful for web frontends
    /// Arguments:
    /// * `data` - serialized JSON to be used
    /// for the game configuration.
    ///
    /// ## Example:
    /// ```rust
    /// # use nightrunner_lib::config::Config;
    /// # let data = nightrunner_lib::util::test_helpers::mock_json_data();
    /// let config = Config::from_json(&data);
    /// ```
    ///
    /// Example valid JSON:
    /// ```rust
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
    ///       "description": "a subject description"
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
    ///   "room_blueprints": [
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
    ///       "item_ids": [
    ///         1,
    ///         2
    ///       ],
    ///       "narrative": 1,
    ///       "subject_ids": [
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
    ///       "item_ids": [],
    ///       "narrative": 2,
    ///       "subject_ids": []
    ///     }
    ///   ]
    /// }"#;
    /// ```
    pub fn from_json(data: &str) -> Config {
        let config_data: ConfigData = serde_json::from_str(data).unwrap();
        let mut items = config_data.items;
        let mut verbs = config_data.allowed_verbs;
        let mut subjects = config_data.subjects;
        let mut narratives = config_data.narratives;
        let mut events = config_data.events;
        let mut room_blueprints = config_data.room_blueprints;

        room_blueprints.sort_by(|a, b| a.id.cmp(&b.id));
        events.sort_by(|a, b| a.id.cmp(&b.id));
        verbs.sort();
        items.sort();
        subjects.sort();
        narratives.sort();

        Config {
            allowed_verbs: verbs,
            allowed_prepositions: AllowedPrepositions::init(),
            allowed_determiners: AllowedDeterminers::init(),
            allowed_movements: AllowedMovements::init(),
            allowed_directions: AllowedDirections::init(),
            items,
            subjects,
            narratives,
            events,
            intro: config_data.intro,
            room_blueprints,
        }
    }
    /// # Config::init_yaml
    /// Load config files from yaml files.
    ///
    /// This is useful for command line frontends
    /// and will read the path and try to load the
    /// files from the path.
    ///
    /// Arguments:
    /// * `path` - serialized yaml
    ///
    /// Required files:
    /// * `allowed_verbs.yml`
    /// * `items.yml`
    /// * `subjects.yml`
    /// * `narratives.yml`
    /// * `events.yml`
    /// * `intro.yml`
    /// * `rooms.yml`
    ///
    /// ## Example:
    /// ```rust
    /// # use nightrunner_lib::config::Config;
    /// let config = Config::from_path("./fixtures/");
    /// ```
    ///
    /// For examples of valid yaml files see the
    /// fixtures directory used for unit tests.
    pub fn from_path(path: &str) -> Config {
        let error_message = format!("Could not find config file at {}", path);
        let narratives_config =
            std::fs::read_to_string(format!("{}narratives.yml", path)).expect(&error_message);
        let items_config =
            std::fs::read_to_string(format!("{}items.yml", path)).expect(&error_message);
        let rooms_config =
            std::fs::read_to_string(format!("{}rooms.yml", path)).expect(&error_message);
        let allowed_verbs_config =
            std::fs::read_to_string(format!("{}verbs.yml", path)).expect(&error_message);
        let subjects_config =
            std::fs::read_to_string(format!("{}subjects.yml", path)).expect(&error_message);
        let events_config =
            std::fs::read_to_string(format!("{}events.yml", path)).expect(&error_message);
        let intro_config =
            std::fs::read_to_string(format!("{}intro.yml", path)).expect(&error_message);

        let mut narratives: Vec<Narrative> = serde_yaml::from_str(&narratives_config[..]).unwrap();
        narratives.sort_by(|a, b| a.id.cmp(&b.id));

        let mut items: Vec<Item> = serde_yaml::from_str(&items_config[..]).unwrap();
        items.sort_by(|a, b| a.id.cmp(&b.id));

        let mut room_blueprints: Vec<RoomBlueprint> =
            serde_yaml::from_str(&rooms_config[..]).unwrap();
        room_blueprints.sort_by(|a, b| a.id.cmp(&b.id));

        let mut events: Vec<Event> = serde_yaml::from_str(&events_config[..]).unwrap();
        events.sort_by(|a, b| a.id.cmp(&b.id));

        let mut subjects: Vec<Subject> = serde_yaml::from_str(&subjects_config[..]).unwrap();
        subjects.sort_by(|a, b| a.id.cmp(&b.id));

        let mut verbs: Vec<Verb> = serde_yaml::from_str(&allowed_verbs_config[..]).unwrap();
        verbs.sort_by(|a, b| a.id.cmp(&b.id));

        let intro: String = serde_yaml::from_str(&intro_config[..]).unwrap();

        Config {
            allowed_determiners: AllowedDeterminers::init(),
            allowed_prepositions: AllowedPrepositions::init(),
            allowed_movements: AllowedMovements::init(),
            allowed_directions: AllowedDirections::init(),
            allowed_verbs: verbs,
            items,
            subjects,
            narratives,
            events,
            intro,
            room_blueprints,
        }
    }
}

/// This struct represents the current state of the game.
/// It contains all the data that is needed to parse raw
/// string inputs into commands, and return the appropriate
/// responses.

#[derive(Debug, Clone, PartialEq)]
pub struct State {
    /// Current value of the input box
    pub input: String,
    /// Player's current location
    pub current_room: u16,
    /// Player's current state
    pub player: Player,
    /// While the config struct provides the available rooms,
    /// the state struct contains the room structs fullly populated.
    /// This is so we can keep track of updates to the room structs
    /// but keep the config struct clean.
    pub rooms: Vec<Room>,
    /// This Config struct holds all the game data
    /// such as verbs, items, etc.
    pub config: Config,
}

impl State {
    /// Takes a config struct and populates the state struct.
    ///
    /// ## Example:
    /// ```rust
    /// # use nightrunner_lib::config::{Config, State};
    /// # use nightrunner_lib::util::test_helpers::mock_json_data;
    /// # let json_data = mock_json_data();
    /// // Using yaml config files from a path
    /// let config1 = Config::from_path("./fixtures/");
    /// let state1 = State::init(config1);
    /// // or using JSON data from a front-end
    /// let config2 = Config::from_json(&json_data);
    /// let state2 = State::init(config2);
    /// ```
    pub fn init(config: Config) -> Self {
        let items = &config.items;
        let subjects = &config.subjects;
        let events = &config.events;
        // let rooms = config
        //     .room_blueprints
        //     .iter()
        //     .cloned()
        //     .map(|blueprint| blueprint.build(events, items, subjects))
        //     .collect::<Vec<Room>>();
        let room_blueprints = &config.room_blueprints;
        let rooms = Room::build_rooms(room_blueprints, events, items, subjects);
        // .iter()
        // .map(|room_blueprint| {
        //     let mut room = Room {
        //         id: room_blueprint.id,
        //         name: room_blueprint.name.clone(),
        //         description: room_blueprint.description.clone(),
        //         exits: room_blueprint.exits.clone(),
        //         narrative: room_blueprint.narrative.clone(),
        //         subjects: vec![],
        //         stash: Storage::default(),
        //         events: vec![],
        //     };
        //     for item_id in &room_blueprint.item_ids {
        //         items.iter().find(|item| item.id == *item_id).map(|item| {
        //             room.stash.add_item(item.clone());
        //         });
        //     }
        //     for subject_id in &room_blueprint.subject_ids {
        //         subjects
        //             .iter()
        //             .find(|subject| subject.id == *subject_id)
        //             .map(|subject| {
        //                 room.subjects.push(subject.clone());
        //             });
        //     }
        //     for event_id in &room_blueprint.room_events {
        //         events
        //             .iter()
        //             .find(|event| event.id == *event_id)
        //             .map(|event| {
        //                 room.events.push(event.clone());
        //             });
        //     }
        //     room
        // })
        // .collect();
        // for blueprint in &mut room_blueprints {
        //     for room_item_id in &mut blueprint.item_ids {
        //         if items.iter().any(|i| i.id == *room_item_id) {
        //             room.stash.items.push(
        //                 items
        //                     .iter()
        //                     .find(|i| i.id == *room_item_id)
        //                     .unwrap()
        //                     .to_owned(),
        //             );
        //         }
        //     }
        // }
        // let state =
        Self {
            input: String::new(),
            current_room: 1,
            player: Player {
                inventory: Storage::default(),
            },
            rooms,
            config,
        }
        // Rc::new(RefCell::new(state))
    }
    /// Returns a clone of the current narrative for the current room.
    pub fn get_narrative(&self) -> Narrative {
        let room = self
            .rooms
            .iter()
            .find(|r| r.id == self.current_room)
            .unwrap();
        let narrative = self
            .config
            .narratives
            .iter()
            .find(|n| n.id == room.narrative)
            .unwrap();
        narrative.clone()
    }
    /// Sets the current room's narrative
    pub fn set_narrative(&mut self, narrative_id: u16) {
        let room = self
            .rooms
            .iter_mut()
            .find(|r| r.id == self.current_room)
            .unwrap();
        room.narrative = narrative_id;
    }
    /// Checks if an event is completed.
    pub fn is_event_completed(&self, event_id: u16) -> bool {
        for room in self.rooms.iter() {
            if let Some(event) = room.events.iter().find(|e| e.id == event_id) {
                return event.completed;
            }
        }
        false
        // self.config
        //     .events
        //     .iter()
        //     .find(|e| e.id == event_id)
        //     .map(|e| e.completed)
        //     .unwrap_or(false)
    }
    /// Marks an event as completed.
    pub fn complete_event(&mut self, event_id: u16) {
        for room in self.rooms.iter_mut() {
            if let Some(event) = room.events.iter_mut().find(|e| e.id == event_id) {
                event.completed = true;
            }
        }
    }
    /// Moves a subject to a different room.
    pub fn move_subject(&mut self, subject_id: u16, location: u16) -> NRResult<()> {
        self.remove_subject(subject_id)?;
        let subject = self
            .config
            .subjects
            .iter()
            .find(|s| s.id == subject_id)
            .ok_or(InvalidSubject)?;
        self.rooms
            .iter_mut()
            .find(|r| r.id == location)
            .ok_or(InvalidRoom)?
            .add_subject(subject.clone());
        Ok(())
    }
    /// Removes a subject from the current room.
    pub fn remove_subject(&mut self, subject_id: u16) -> NRResult<()> {
        let current_room = self
            .rooms
            .iter_mut()
            .find(|r| r.id == self.current_room)
            .ok_or(InvalidRoom)?;
        current_room.remove_subject(subject_id);
        Ok(())
    }
    /// Adds a subject to the current room.
    pub fn add_subject(&mut self, subject: Subject) -> NRResult<()> {
        let current_room = self
            .rooms
            .iter_mut()
            .find(|r| r.id == self.current_room)
            .ok_or(InvalidRoom)?;
        current_room.add_subject(subject);
        Ok(())
    }
}

/// This struct represents the storage for both the player
/// and the room and implements functions to add and remove
/// items from the storage.
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Deserialize, Serialize, Default)]
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
    // pub item_ids: Vec<u16>,
}

impl Storage {
    /// This function adds an item to the storage.
    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }
    /// This function removes an item from the storage
    /// if availabl and returns the item removed. This
    /// is so that the same item can be added to another
    /// storage, for example when the user drops an item
    /// from their inventory or picks up an item from
    /// the room.
    pub fn remove_item(&mut self, item: Item) -> NRResult<Item> {
        let target_item = self.items.iter().position(|i| i.name == item.name);
        match target_item {
            Some(item_index) => Ok(self.items.remove(item_index)),
            None => Err(NoItem.into()),
        }
    }
}

/// This struct represents the player's current state.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Player {
    /// The player's inventory
    pub inventory: Storage,
}

#[cfg(test)]
mod tests;
