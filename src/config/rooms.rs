use crate::{config::directions::Directions, parser::errors::NoItem, NRResult};
use serde::{Deserialize, Serialize};

/// This struct represents a room in the game.
#[derive(Debug, Clone, Deserialize, Serialize, Eq, Ord, PartialEq, PartialOrd)]
#[serde(rename_all = "snake_case")]
pub struct Room {
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
    pub exits: Vec<Exits>,
    /// This is the list of items that are
    /// currently in the room.
    pub stash: Storage,
    /// This is the list of events that can
    /// be triggered in this room.
    pub room_events: Vec<u16>,
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
    pub subjects: Vec<u16>,
}

impl Room {
    /// This function checks if the player can move
    /// in the direction specified by the action struct.
    ///
    /// If an exit with the given direction exits, move
    /// the player there.
    pub fn can_move(&mut self, direction: Directions) -> Result<u16, ()> {
        let exits: Vec<&Exits> = self
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
    /// The list of item ids that are currently
    /// available in storage. Only used for the
    /// configuration data.
    pub item_ids: Vec<u16>,
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

/// This struct represents an item in the game.
/// It contains the name of the item, the description
/// and whether or not the item can be picked up.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub struct Item {
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

/// This struct represents exits from a room.
#[derive(Debug, Clone, Deserialize, Serialize, Eq, Ord, PartialEq, PartialOrd)]
#[serde(rename_all = "snake_case")]
pub struct Exits {
    /// The room that the exit leads to.
    pub room_id: u16,
    /// The direction this direction is located.
    pub direction: Directions,
}

#[cfg(test)]
#[path = "rooms_tests.rs"]
mod room_tests;
