pub(crate) mod determiners;
pub(crate) mod directions;
pub(crate) mod movements;
pub(crate) mod prepositions;
pub(crate) mod rooms;

use self::determiners::AllowedDeterminers;
use self::directions::AllowedDirections;
use self::movements::AllowedMovements;
use self::prepositions::AllowedPrepositions;
use self::rooms::{Item, Room, Storage};
use serde::{Deserialize, Serialize};
use serde_json;
use std::cell::RefCell;
use std::rc::Rc;

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
/// ```ignore
/// Verb {
///    id: 1,
///    names: vec!["look", "peek"],
///    verb_function: VerbFunction::Look,
/// }
///
/// Verb {
///    id: 2,
///    names: vec!["take", "pick"],
///    verb_function: VerbFunction::Take,
/// }
///
/// Verb {
///    id: 3,
///    names: vec!["parkour", "flip"],
///    verb_function: VerbFunction::Normal,
/// }
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
    /// anything. For the possible functions see the [VerbFunction](VerbFunction) enum.
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
/// ```ignore
/// Subject {
///    id: 1,
///    name: "person".to_string(),
///    description: "A person dressed all in black".to_string(),
///    default_text: "Person: I'm busy now. Maybe later.".to_string(),
/// }
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
/// - An event that happens in room 1 as a response to
/// talking to subject 2:
/// ```ignore
/// Event {
///   id: 1,
///   location: 1,
///   name: "Talking to subject 1",
///   description: "This event happens when you talk to subject 2.",
///   destination: None,
///   narrative: 2,
///   // here verb id 3 has to be marked with VerbFunction::Talk
///   required_verb: 3,
///   required_subject: 2,
///   required_item: None,
///   completed: false,
///   add_item: None,
///   remove_old_narrative: None,
///   remove_item: None,
///   required_event: Vec::new(),
/// }
/// ```
///
/// - An event that happens in room 2 as a response to
/// using an item with a subject and requires event
/// 1 to be completed:
/// ```ignore
/// Event {
///   id: 2,
///   location: 2,
///   name: "Using item 3 with subject 1",
///   description: "This event happens when you use an item with subject 1.",
///   destination: None,
///   narrative: 4,
///   // here verb id 2 would be marked with VerbFunction::Normal
///   required_verb: 3,
///   required_subject: 1,
///   required_item: 3,
///   completed: false,
///   add_item: None,
///   remove_old_narrative: None,
///   // here item id 3 would be removed after the event is completed
///   remove_item: 3,
///   required_event: Vec::new(),
/// }
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
    /// If the event narrative is supposed to replace
    /// the text currently displayed on the screen,
    /// this needs to be set to true.
    /// This is useful to avoid a lot of screen scrolling
    /// when the event narrative is long.
    pub remove_old_narrative: bool,
    /// If the event removes an item from the inventory,
    /// this is the item id.
    pub remove_item: Option<u16>,
    /// If the event requires other events to be completed,
    /// this is a list of event ids that need to be completed
    /// before this event can be triggered.
    pub required_events: Vec<u16>,
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
    rooms: Vec<Room>,
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
    /// All the possible rooms in the game.
    pub rooms: Vec<Room>,
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
            rooms: Vec::new(),
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
    /// ```ignore
    /// let data = crate::utils::mock_data();
    /// let config = Config::init_json(data);
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
    /// ```
    pub fn from_json(data: &str) -> Config {
        let config_data: ConfigData = serde_json::from_str(data).unwrap();
        let mut items = config_data.items;
        let mut verbs = config_data.allowed_verbs;
        let mut subjects = config_data.subjects;
        let mut narratives = config_data.narratives;
        let mut events = config_data.events;
        let mut rooms = config_data.rooms;

        rooms.sort_by(|a, b| a.id.cmp(&b.id));
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
            items: items,
            subjects: subjects,
            narratives: narratives,
            events: events,
            intro: config_data.intro,
            rooms: rooms,
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
    /// ```ignore
    /// let config = Config::init_yaml("/path/to/config");
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

        let mut rooms: Vec<Room> = serde_yaml::from_str(&rooms_config[..]).unwrap();
        rooms.sort_by(|a, b| a.id.cmp(&b.id));

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
            rooms,
        }
    }
}

/// This struct represents the current state of the game.
/// It contains all the data that is needed to parse raw
/// string inputs into commands, and return the appropriate
/// responses.
///
/// Whil you can create this struct manually, this isn't
/// recommended. Instead, use the State::init(config) function.
///
/// An example of what this struct looks like:
/// ```ignore
/// State {
///     input: "",
///     current_room: 1,
///     player: Player {
///         inventory: Storage {
///             items: vec![],
///             item_ids: vec![],
///         },
///     }
///     rooms: vec![Room {
///         id: 1,
///         name: "room",
///         description: "a room",
///         exits: vec![Exits {
///             direction: Direction::North,
///             room_id: 2,
///         }],
///         stash: Storage {
///             items: vec![Item {
///                 id: 1,
///                 name: "item",
///                 description: "an item",
///                 can_pick: true,
///             }],
///             item_ids: vec![1],
///         },
///         room_events: vec![1],
///         narrative: 1,
///         subjects: vec![1],
///     }],
///     config: Config {
///         // These fields are automatically populated
///         allowed_prepositions: AllowedPrepositions {
///             prepositions: vec![String::new()],
///         },
///         allowed_determiners: AllowedDeterminers {
///             determiners: vec![String::new()],
///         },
///         allowed_movements: AllowedMovements {
///             movements: vec![String::new()],
///         },
///         allowed_directions: AllowedDirections {
///             directions: vec![Direction::North],
///         },
///         allowed_verbs: vec![Verb {
///                 id: 1,
///                 names: ["verb"],
///                 verb_function: "look",
///             }],
///         items:  vec![Item {
///                 id: 1,
///                 name: "item",
///                 description: "an item",
///                 can_pick: true,
///             }],
///          subjects: vec![Subject {
///              id: 1,
///              name: "subject",
///              description: "a subject",
///              default_text: "default text",
///          }],
///          narratives: vec![Narrative {
///              id: 1,
///              text: "a narrative",
///          }],
///          events: vec![Event {
///              id: 1,
///              name: "event",
///              description: "an event",
///              destination: 1,
///              narrative: 1,
///              required_verb: 2,
///              required_subject: 1,
///              required_item: 1
///              completed: false
///              add_item: 1
///              remove_old_narrative: false
///              remove_item: 2
///              required_events: [1]
///          }],
///         intro: "intro",
///     }
/// }
/// ```

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
    /// ```ignore
    /// // Using yaml config files from a path
    /// let config = Config::from_path("config/");
    /// // or using JSON data from a front-end
    /// let config = Config::from_json(json_data);
    /// let state = State::new(config);
    /// ```
    pub fn init(config: Config) -> Rc<RefCell<Self>> {
        let items = &config.items;
        let mut rooms = config.clone().rooms;
        for room in &mut rooms {
            for room_item_id in &mut room.stash.item_ids {
                if items
                    .iter()
                    .find(|i| &i.id == &room_item_id.clone())
                    .is_some()
                {
                    room.stash.items.push(
                        items
                            .iter()
                            .find(|i| &i.id == &room_item_id.clone())
                            .unwrap()
                            .to_owned(),
                    );
                }
            }
        }
        let state = Self {
            input: String::new(),
            current_room: 1,
            player: Player {
                inventory: Storage {
                    items: vec![],
                    item_ids: vec![],
                },
            },
            rooms,
            config: config,
        };
        Rc::new(RefCell::new(state))
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
