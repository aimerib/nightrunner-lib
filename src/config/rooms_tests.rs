use super::*;
use crate::{config::directions::Directions, config::rooms::Room};
#[cfg(test)]
use pretty_assertions::assert_eq;

#[test]
fn it_can_move() {
    let mut room = Room {
        id: 1,
        name: "text".to_owned(),
        description: "text".to_owned(),
        exits: vec![Exits {
            room_id: 2,
            direction: Directions::NORTH,
        }],
        stash: Storage {
            items: vec![],
            item_ids: vec![],
        },
        room_events: vec![],
        narrative: 1,
        subjects: vec![],
    };
    let room_id = room.can_move(Directions::NORTH);
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
            direction: Directions::NORTH,
        }],
        stash: Storage {
            items: vec![],
            item_ids: vec![],
        },
        room_events: vec![],
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
            direction: Directions::NORTH,
        }],
        stash: Storage {
            items: vec![item.clone()],
            item_ids: vec![],
        },
        room_events: vec![],
        narrative: 1,
        subjects: vec![],
    };

    let remove_result = room.stash.remove_item(item.clone());
    assert_eq!(remove_result.unwrap(), item.clone());
    assert_eq!(room.stash.items.len(), 0);
    let remove_error = room.stash.remove_item(item.clone());
    assert_eq!(
        remove_error.unwrap_err().to_string(),
        "You're not carrying that.".to_string()
    );
}
