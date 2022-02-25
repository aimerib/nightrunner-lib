use super::*;
use crate::{
    config::directions::Directions,
    config::{rooms::Room, Item},
};
#[cfg(test)]
use pretty_assertions::assert_eq;

#[test]
fn it_builds_a_room() {
    let bp = RoomBlueprint {
        id: 1,
        name: "Test Room".to_string(),
        description: "This is a test room.".to_string(),
        exits: vec![Exits {
            direction: Directions::North,
            room_id: 2,
        }],
        item_ids: vec![1],
        narrative: 1,
        subject_ids: vec![],
    };
    let rooms = Room::build_rooms(
        &[bp],
        &[],
        &[Item {
            id: 1,
            name: "item1".to_string(),
            description: "item1".to_string(),
            can_pick: false,
        }],
        &[],
    );
    let manual_room = Room {
        id: 1,
        name: "Test Room".to_owned(),
        description: "This is a test room.".to_owned(),
        exits: vec![Exits {
            room_id: 2,
            direction: Directions::North,
        }],
        stash: Storage {
            items: vec![Item {
                id: 1,
                name: "item1".to_string(),
                description: "item1".to_string(),
                can_pick: false,
            }],
        },
        events: vec![],
        narrative: 1,
        subjects: vec![],
    };
    assert_eq!(1, rooms.len());
    assert_eq!(manual_room, rooms[0]);
    assert_eq!("This is a test room.", rooms[0].description);
    assert!(rooms[0].stash.items[0].name == "item1");
}

#[test]
fn it_can_move() {
    let mut room = Room {
        id: 1,
        name: "text".to_owned(),
        description: "text".to_owned(),
        exits: vec![Exits {
            room_id: 2,
            direction: Directions::North,
        }],
        stash: Storage { items: vec![] },
        events: vec![],
        narrative: 1,
        subjects: vec![],
    };
    let room_id = room.can_move(Directions::North);
    assert_eq!(room_id, Ok(2));
}
#[test]
fn it_adds_item() {
    let mut room = Room {
        id: 1,
        name: "text".to_owned(),
        description: "text".to_owned(),
        exits: vec![Exits {
            room_id: 2,
            direction: Directions::North,
        }],
        stash: Storage { items: vec![] },
        events: vec![],
        narrative: 1,
        subjects: vec![],
    };
    let item = Item {
        id: 1,
        name: "text".to_owned(),
        description: "text".to_owned(),
        can_pick: true,
    };
    room.stash.add_item(item.clone());
    assert_eq!(room.stash.items.len(), 1);
    assert_eq!(room.stash.items[0], item);
}
#[test]
fn it_removes_item() {
    let item = Item {
        id: 1,
        name: "text".to_owned(),
        description: "text".to_owned(),
        can_pick: true,
    };
    let mut room = Room {
        id: 1,
        name: "text".to_owned(),
        description: "text".to_owned(),
        exits: vec![Exits {
            room_id: 2,
            direction: Directions::North,
        }],
        stash: Storage {
            items: vec![item.clone()],
        },
        events: vec![],
        narrative: 1,
        subjects: vec![],
    };

    let remove_result = room.stash.remove_item(item.clone());
    assert_eq!(remove_result.unwrap(), item);
    assert_eq!(room.stash.items.len(), 0);
    let remove_error = room.stash.remove_item(item);
    assert_eq!(
        remove_error.unwrap_err().to_string(),
        "You're not carrying that.".to_string()
    );
}

#[test]
fn it_adds_subject() {
    let subject = Subject {
        id: 1,
        name: "text".to_owned(),
        description: "text".to_owned(),
        default_text: "default text".to_owned(),
    };
    let mut room = Room {
        id: 1,
        name: "text".to_owned(),
        description: "text".to_owned(),
        exits: vec![Exits {
            room_id: 2,
            direction: Directions::North,
        }],
        stash: Storage { items: vec![] },
        events: vec![],
        narrative: 1,
        subjects: vec![],
    };
    room.add_subject(subject);
    assert!(!room.subjects.is_empty());
    assert_eq!(room.subjects[0].name, "text");
}

#[test]
fn it_removes_subject() {
    let mut room = Room {
        id: 1,
        name: "text".to_owned(),
        description: "text".to_owned(),
        exits: vec![Exits {
            room_id: 2,
            direction: Directions::North,
        }],
        stash: Storage { items: vec![] },
        events: vec![],
        narrative: 1,
        subjects: vec![Subject {
            id: 1,
            name: "text".to_owned(),
            description: "text".to_owned(),
            default_text: "default text".to_owned(),
        }],
    };
    assert!(!room.subjects.is_empty());
    room.remove_subject(1);
    assert!(room.subjects.is_empty());
}
