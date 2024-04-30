pub(crate) mod determiners;
#[allow(non_snake_case)]
pub(crate) mod directions;
pub(crate) mod movements;
pub(crate) mod prepositions;


#[allow(non_snake_case)]
pub(crate) mod types;

use crate::parser::errors::{InvalidRoom, InvalidSubject, NoItem};
use crate::NRResult;

use self::determiners::AllowedDeterminers;
use self::directions::{AllowedDirections, Directions};
use self::movements::AllowedMovements;
use self::prepositions::AllowedPrepositions;
use serde::{Deserialize, Serialize};
use serde_json;


pub use types::{ Exit, RoomBlueprint, Narrative, Verb, VerbFunction, Subject, Event, Item, Room, Storage, Player };


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

impl std::fmt::Display for Subject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.name[..])
    }
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

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.name[..])
    }
}

impl Room {
    /// This function checks if the player can move
    /// in the direction specified by the action struct.
    ///
    /// If an exit with the given direction exits, move
    /// the player there.
    pub fn can_move(&mut self, direction: Directions) -> Result<u16, ()> {
        let exits: Vec<&Exit> = self
            .exits
            .iter()
            .filter(|exit| exit.direction == direction)
            .collect();
        if !exits.is_empty() {
            Ok(exits[0].room_id)
        } else {
            Err(())
        }
    }
    /// This function adds a subject to the room.
    pub fn add_subject(&mut self, subject: Subject) {
        self.subjects.push(subject);
    }
    /// This function removes a subject from the room.
    pub fn remove_subject(&mut self, subject_id: u16) {
        self.subjects.retain(|s| s.id != subject_id);
    }

    /// This function takes a list of room blueprints
    /// and a list of events, items, and subjects
    /// and creates a list of rooms from them.
    pub fn build_rooms(
        blueprints: &[RoomBlueprint],
        events: &[Event],
        items: &[Item],
        subjects: &[Subject],
    ) -> Vec<Room> {
        blueprints
            .iter()
            .map(|room_blueprint| {
                let mut room = Room {
                    id: room_blueprint.id,
                    name: room_blueprint.name.clone(),
                    description: room_blueprint.description.clone(),
                    exits: room_blueprint.exits.clone(),
                    narrative: room_blueprint.narrative,
                    subjects: vec![],
                    stash: Storage::default(),
                    events: vec![],
                };
                for item_id in &room_blueprint.item_ids {
                    if let Some(item) = items.iter().find(|item| item.id == *item_id) {
                        room.stash.add_item(item.clone());
                    }
                }
                for subject_id in &room_blueprint.subject_ids {
                    if let Some(subject) = subjects.iter().find(|subject| subject.id == *subject_id)
                    {
                        room.subjects.push(subject.clone());
                    }
                }
                for event in events {
                    if event.location == room_blueprint.id {
                        room.events.push(event.clone());
                    }
                }
                room
            })
            .collect::<Vec<Room>>()
    }
    /// Returns the RoomBlueprint for the current room.
    pub fn into_room_blueprint(&self) -> RoomBlueprint {
        RoomBlueprint {
            id: self.id,
            name: self.name.clone(),
            description: self.description.clone(),
            exits: self.exits.clone(),
            item_ids: self.stash.items.iter().map(|i| i.id).collect(),
            narrative: self.narrative,
            subject_ids: self.subjects.iter().map(|s| s.id).collect(),
        }
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
    /// * `room_blueprints.yml`
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
        let room_blueprints_config =
            std::fs::read_to_string(format!("{}room_blueprints.yml", path)).expect(&error_message);
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
            serde_yaml::from_str(&room_blueprints_config[..]).unwrap();
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
        let room_blueprints = &config.room_blueprints;
        let rooms = Room::build_rooms(room_blueprints, events, items, subjects);
        Self {
            input: String::new(),
            current_room: 1,
            player: Player {
                inventory: Storage::default(),
            },
            rooms,
            config,
        }
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

#[cfg(test)]
mod tests;

#[cfg(test)]
#[path = "rooms_tests.rs"]
mod room_tests;
