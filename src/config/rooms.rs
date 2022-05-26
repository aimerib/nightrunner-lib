use serde::{Deserialize, Serialize};

use super::{directions::Directions, Event, Item, Storage, Subject};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct RoomBlueprint {
    pub(crate) id: u16,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) exits: Vec<Exits>,
    pub(crate) item_ids: Vec<u16>,
    pub(crate) narrative: u16,
    pub(crate) subject_ids: Vec<u16>,
}

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

    pub fn add_subject(&mut self, subject: Subject) {
        self.subjects.push(subject);
    }
    pub fn remove_subject(&mut self, subject_id: u16) {
        self.subjects.retain(|s| s.id != subject_id);
    }

    pub(crate) fn build_rooms(
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
