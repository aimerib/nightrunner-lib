use super::super::interpreter::*;
use crate::config::{Config, Verb};
#[cfg(test)]
use pretty_assertions::assert_eq;
#[test]
fn it_looks_at_room() {
    let config = Config::from_path("fixtures/");
    let mut state = State::init(config);
    let look_result1 = look_room(&state);
    state.current_room = 7;
    let look_result2 = look_room(&state);
    assert!(look_result1.is_ok());
    assert_eq!(
        look_result1.unwrap(),
        ParsingResult::Look(
            "first room\n\nHere you see: \nan item1\nan item2\nsubject1".to_string()
        )
    );
    assert!(look_result2.is_err());
    assert_eq!(look_result2.unwrap_err().to_string(), NoRoom.to_string());
}
#[test]
fn it_looks_at_subject() {
    let config = Config::from_path("fixtures/");
    let state = State::init(config);
    let subject1 = state.config.subjects[0].clone();
    let subject2 = Subject {
        id: 7,
        name: "subject2".to_string(),
        description: "a non-existing subject".to_string(),
        default_text: "".to_string(),
    };
    let look_result1 = look_subject(&state, subject1);
    let look_result2 = look_subject(&state, subject2);
    assert!(look_result1.is_ok());
    assert_eq!(
        look_result1.unwrap(),
        ParsingResult::Look("a subject description".to_string())
    );
    assert_eq!(
        look_result2.unwrap(),
        ParsingResult::Look("I can't see that here".to_string())
    );
}
#[test]
fn it_looks_at_item() {
    let config = Config::from_path("fixtures/");
    let state = State::init(config);
    let item1 = state.config.items[0].clone();
    let item2 = Item {
        id: 7,
        name: "item2".to_string(),
        description: "a non-existing item".to_string(),
        can_pick: false,
    };
    let look_result1 = look_item(&state, item1);
    let look_result2 = look_item(&state, item2);
    assert!(look_result1.is_ok());
    assert_eq!(
        look_result1.unwrap(),
        ParsingResult::Look("item 1 description".to_string())
    );
    assert!(look_result2.is_ok());
    assert_eq!(
        look_result2.unwrap(),
        ParsingResult::Look("I can't see that here".to_string())
    );
}

#[test]
fn it_picks_items() {
    let config = Config::from_path("fixtures/");
    let state = State::init(config);
    let current_room_id = state.current_room;
    let current_room = state
        .rooms
        .iter()
        .find(|room| room.id == current_room_id)
        .unwrap()
        .clone();
    let item1 = current_room.stash.items[0].clone();
    let item2 = current_room.stash.items[1].clone();
    let item3 = Item {
        id: 7,
        name: "item3".to_string(),
        description: "a non-existing item".to_string(),
        can_pick: false,
    };
    let pick_result1 = pick_item(&state, item1);
    let pick_result2 = pick_item(&state, item2);
    let pick_result3 = pick_item(&state, item3);

    assert!(pick_result1.is_err());
    assert_eq!(pick_result1.unwrap_err().to_string(), CantPick.to_string());
    assert!(pick_result2.is_ok());
    assert_eq!(
        pick_result2.unwrap().1,
        ParsingResult::NewItem("\nYou now have a item2\n".to_string())
    );
    assert!(&pick_result3.is_err());
    let result3_message = match pick_result3 {
        Ok(_) => panic!("pick_item should have failed"),
        Err(e) => e,
    };
    assert_eq!(result3_message.to_string(), NoItem.to_string());
}

#[test]
fn it_drops_items() {
    let config = Config::from_path("fixtures/");
    let mut state = State::init(config);
    let current_room_id = &state.current_room;
    let current_room = &state
        .rooms
        .iter()
        .find(|room| &room.id == current_room_id)
        .unwrap()
        .clone();
    let item1 = current_room.stash.items[0].clone();
    let item2 = current_room.stash.items[1].clone();
    state.player.inventory.items.push(item1.clone());

    let drop_result1 = drop_item(&state, item1);
    let drop_result2 = drop_item(&state, item2);

    assert!(drop_result1.is_ok());
    assert_eq!(
        drop_result1.unwrap().1,
        ParsingResult::DropItem("\nYou no longer have a item1\n".to_string())
    );
    assert!(&drop_result2.is_err());
    let result2_message = match drop_result2 {
        Ok(_) => panic!("pick_item should have failed"),
        Err(e) => e,
    };
    assert_eq!(result2_message.to_string(), NoItem.to_string());
}

#[test]
fn it_shows_inventory() {
    let config = Config::from_path("fixtures/");
    let mut state = State::init(config);

    let inventory_result_1 = show_inventory(&state);
    assert_eq!(
        inventory_result_1.unwrap(),
        ParsingResult::Inventory("You are not carrying anything.".to_string())
    );
    state.player.inventory.items.push(Item {
        id: 7,
        name: "item1".to_string(),
        description: "item 1 description".to_string(),
        can_pick: false,
    });
    let inventory_result2 = show_inventory(&state);
    assert_eq!(
        inventory_result2.unwrap(),
        ParsingResult::Inventory("You are currently carrying: \n\nan item1".to_string())
    );
    state.player.inventory.items.push(Item {
        id: 8,
        name: "item2".to_string(),
        description: "item 2 description".to_string(),
        can_pick: false,
    });
    let inventory_result3 = show_inventory(&state);
    assert_eq!(
        inventory_result3.unwrap(),
        ParsingResult::Inventory("You are currently carrying: \n\nan item1\nan item2".to_string())
    );
}

#[test]
fn it_extracts_item_and_subject() {
    let config = Config::from_path("fixtures/");
    let mut state = State::init(config);
    let subject = Subject {
        id: 1,
        name: "subject1".to_string(),
        description: "a subject description".to_string(),
        default_text: "a subject default text".to_string(),
    };
    let item1 = Item {
        id: 1,
        name: "item1".to_string(),
        description: "item 1 description".to_string(),
        can_pick: false,
    };
    state.player.inventory.items.push(item1.clone());
    let item2 = Item {
        id: 2,
        name: "item2".to_string(),
        description: "item 2 description".to_string(),
        can_pick: false,
    };
    let verb = Verb {
        id: 1,
        names: vec!["go".to_string()],
        verb_function: VerbFunction::Normal,
    };
    let action1 = Action {
        verb: Some(verb.clone()),
        subject: Some(subject.clone()),
        item: Some(item1.clone()),
        movement: None,
        command_tokens: vec![
            "go".to_string(),
            "subject1".to_string(),
            "item1".to_string(),
        ],
        input: "go subject1 item1".to_string(),
    };
    let action2 = Action {
        verb: Some(verb),
        subject: Some(subject.clone()),
        item: Some(item2),
        movement: None,
        command_tokens: vec![
            "go".to_string(),
            "subject1".to_string(),
            "item2".to_string(),
        ],
        input: "go subject1 item2".to_string(),
    };
    let action3 = Action {
        verb: None,
        subject: None,
        item: None,
        movement: Some(Directions::North),
        command_tokens: vec!["north".to_string()],
        input: "north".to_string(),
    };
    let extract_result1 = extract_item_subject(&state, &action1);
    let extract_result2 = extract_item_subject(&state, &action2);
    let extract_result3 = extract_item_subject(&state, &action3);
    assert_eq!(extract_result1, (Some(item1), Some(subject.clone())));
    assert_eq!(extract_result2, (None, Some(subject)));
    assert_eq!(extract_result3, (None, None));
}

#[test]
fn it_handles_movement() {
    let config = Config::from_path("fixtures/");
    let state = State::init(config);
    let (new_state, movement_result1) = handle_movement(&state, Some(Directions::South)).unwrap();
    let movement_result2 = handle_movement(&new_state, Some(Directions::South)).unwrap_err();
    let movement_result3 = handle_movement(&new_state, None).unwrap_err();
    let mut message_parts = HashMap::new();
    message_parts.insert(MessageParts::RoomText, "this is a templated which exists in the game item3.\n\nthis is a templated subject that exists in the game subject2.".to_string());
    message_parts.insert(
        MessageParts::Exits,
        "Exits:\nto the north you see first room".to_string(),
    );
    message_parts.insert(MessageParts::EventText, "".to_string());
    assert_eq!(
        movement_result1,
        ParsingResult::EventSuccess(EventMessage{
            message: "this is a templated which exists in the game item3.\n\nthis is a templated subject that exists in the game subject2.\n\n\nExits:\nto the north you see first room".to_string(),
            templated_words: vec!["item3".to_string(), "subject2".to_string()],
            message_parts,
        })
    );
    assert_eq!(movement_result2.to_string(), InvalidMovement.to_string());
    assert_eq!(movement_result3.to_string(), InvalidMovement.to_string());
}

#[test]
fn it_handles_verbs() {
    let config = Config::from_path("fixtures/");
    let state = State::init(config);
    let action_north = Action::parse(&state, "north");
    let action_look = Action::parse(&state, "look");
    let action_inventory = Action::parse(&state, "i");
    let action_quit = Action::parse(&state, "quit");
    let action_help = Action::parse(&state, "help");
    let verb_result1 = handle_verb(&state, action_north);
    let verb_result2 = handle_verb(&state, action_look);
    let verb_result3 = handle_verb(&state, action_inventory);
    let verb_result4 = handle_verb(&state, action_quit);
    let verb_result5 = handle_verb(&state, action_help);
    assert_eq!(
        verb_result1.unwrap_err().to_string(),
        InvalidVerb.to_string()
    );
    assert_eq!(
        verb_result2.unwrap().1,
        ParsingResult::Look(String::from(
            "first room\n\nHere you see: \nan item1\nan item2\nsubject1"
        ))
    );
    assert_eq!(
        verb_result3.unwrap().1,
        ParsingResult::Inventory(String::from("You are not carrying anything."))
    );
    assert_eq!(verb_result4.unwrap().1, ParsingResult::Quit);
    assert_eq!(verb_result5.unwrap().1, ParsingResult::Help(String::from("\nTo play this game you type your commands and hit enter to execute them. Typically a command has at most three parts: a verb, a subject, and an item. A verb indicates an action you, the player, wants to execute. Many commands can be executed with just a verb such as look, help, quit. For more complex commands you will also need verb and either a subject or an item. A command can also have a verb, item, and subject. A complex command can be: look at dog, talk to person, pick the box, give the box to the dog.\n\nThe game will ignore words like 'to', 'the', 'at', 'from', so using them is optional. A valid command can be: talk person, pick box, go south, climb tree, use axe tree.\n\nValid verbs: quit, help, look, inventory, pick, drop, give, talk, hug")));
}

#[test]
fn it_handles_verb_items() {
    let config = Config::from_path("fixtures/");
    let state = State::init(config);
    let action_look_item = Action::parse(&state, "look item1");
    let action_pick_item = Action::parse(&state, "pick item2");
    let action_cant_pick_item = Action::parse(&state, "pick item1");
    let action_drop_item = Action::parse(&state, "drop item2");
    let action_wrong_verb = Action::parse(&state, "quit item1");
    // Returns the item description if item is in the room
    let (new_state, action_look_item_result) = handle_verb_item(&state, action_look_item).unwrap();
    // Adds the item to the player's inventory and returns a message if the item can be picked
    let (new_state, action_pick_item_result) =
        handle_verb_item(&new_state, action_pick_item).unwrap();
    // Returns an error if the item can't be picked
    let action_cant_pick_item_result =
        handle_verb_item(&new_state, action_cant_pick_item).unwrap_err();
    // Removes the item from the player's inventory and returns a message if the item is in the inventory
    let (new_state, action_drop_item_result) =
        handle_verb_item(&new_state, action_drop_item).unwrap();
    // Returns an error if the verb is not valid
    let action_wrong_verb_result = handle_verb_item(&new_state, action_wrong_verb).unwrap_err();
    assert_eq!(
        action_look_item_result,
        ParsingResult::Look(String::from("item 1 description"))
    );
    assert_eq!(
        action_pick_item_result,
        ParsingResult::NewItem(String::from("\nYou now have a item2\n"))
    );
    assert_eq!(
        action_cant_pick_item_result.to_string(),
        CantPick.to_string()
    );
    assert_eq!(
        action_drop_item_result,
        ParsingResult::DropItem(String::from("\nYou no longer have a item2\n"))
    );
    assert_eq!(
        action_wrong_verb_result.to_string(),
        InvalidVerb.to_string()
    );
}

#[test]
fn it_handles_verb_subjects() {
    let config = Config::from_path("fixtures/");
    let state = State::init(config);

    let action_talk_subject = Action::parse(&state, "talk item1");
    let action_talk_subject2 = Action::parse(&state, "talk subject1");
    let action_look_subject = Action::parse(&state, "look subject1");
    let action_give_subject = Action::parse(&state, "give subject1");

    let action_talk_subject_result = handle_verb_subject(&state, action_talk_subject);
    let action_talk_subject2_result = handle_verb_subject(&state, action_talk_subject2);
    let action_look_subject_result = handle_verb_subject(&state, action_look_subject);
    let action_give_subject_result = handle_verb_subject(&state, action_give_subject);
    let mut message_parts = HashMap::new();
    message_parts.insert(MessageParts::RoomText, "text".to_string());
    message_parts.insert(
        MessageParts::Exits,
        "Exits:\nto the south you see second room".to_string(),
    );
    message_parts.insert(MessageParts::EventText, "".to_string());
    assert_eq!(
        action_talk_subject_result.unwrap_err().to_string(),
        InvalidSubject.to_string()
    );
    // This is parsed as an event, so an EventMessage is returned
    assert_eq!(
        action_talk_subject2_result.unwrap().1,
        ParsingResult::EventSuccess(EventMessage {
            message: String::from("text\n\n\nExits:\nto the south you see second room"),
            templated_words: vec![],
            message_parts,
        })
    );
    assert_eq!(
        action_look_subject_result.unwrap().1,
        ParsingResult::Look(String::from("a subject description"))
    );
    assert_eq!(
        action_give_subject_result.unwrap().1,
        ParsingResult::SubjectNoEvent("default text".to_string())
    );
}

#[test]
fn it_handles_events() {
    let config = Config::from_path("fixtures/");
    let state = State::init(config);
    let action_talk_subject = Action::parse(&state, "talk subject1");
    let action_give_subject = Action::parse(&state, "give subject1");
    let action_hug_subject = Action::parse(&state, "hug subject1");
    let action_talk_subject2 = Action::parse(&state, "talk subject2");
    let action_give_item = Action::parse(&state, "give item2 subject1");

    // If we try to complete event 2 before event one it should return an error
    let action_hug_subject_result = handle_event(&state, action_hug_subject.clone()).unwrap_err();
    assert_eq!(
        action_hug_subject_result.to_string(),
        RequiredEventNotCompleted.to_string()
    );

    let (new_state, action_talk_subject_result) =
        handle_event(&state, action_talk_subject).unwrap();
    let mut message_parts = HashMap::new();
    message_parts.insert(MessageParts::EventText, "".to_string());
    message_parts.insert(MessageParts::RoomText, "text".to_string());
    message_parts.insert(
        MessageParts::Exits,
        "Exits:\nto the south you see second room".to_string(),
    );
    assert_eq!(
        action_talk_subject_result,
        ParsingResult::EventSuccess(EventMessage {
            message: String::from("text\n\n\nExits:\nto the south you see second room"),
            templated_words: vec![],
            message_parts,
        })
    );
    assert_eq!(
        new_state
            .rooms
            .iter()
            .flat_map(|room| &room.events)
            .filter(|e| e.completed)
            .map(|e| e.id)
            .collect::<Vec<u16>>(),
        vec![4]
    );
    assert_ne!(
        new_state
            .rooms
            .iter()
            .flat_map(|room| &room.events)
            .filter(|e| e.completed)
            .map(|e| e.id)
            .collect::<Vec<u16>>(),
        vec![2, 4],
        "Event 2 should not be completed {:#?}",
        &new_state.config.events
    );

    let (new_state, action_give_subject_result) =
        handle_event(&new_state, action_give_subject).unwrap();
    assert_eq!(
        action_give_subject_result,
        ParsingResult::SubjectNoEvent("default text".to_string())
    );

    // Event 2 can be completed now
    let (new_state, action_hug_subject_result) =
        handle_event(&new_state, action_hug_subject).unwrap();
    assert_eq!(
        new_state
            .rooms
            .iter()
            .flat_map(|room| &room.events)
            .filter(|e| e.completed)
            .map(|e| e.id)
            .collect::<Vec<u16>>(),
        vec![2, 4],
        "Event 2 should be completed at this point. {:#?}",
        &new_state.config.events
    );
    assert!(
        new_state
            .rooms
            .iter()
            .flat_map(|room| &room.events)
            .find(|e| e.id == 2)
            .unwrap()
            .completed,
        "Event 2 should be completed at this point. {:#?}",
        &new_state.config.events.iter().find(|e| e.id == 2).unwrap()
    );

    // This event replaces the narrative, so we return a message to the front-end
    // with the new narrative, but we leave the room narrative alone so we can
    // display it again if the player wants to go back to the room.
    let mut message_parts = HashMap::new();
    message_parts.insert(
        MessageParts::RoomText,
        "this narrative should replace the old one.".to_string(),
    );
    message_parts.insert(
        MessageParts::Exits,
        "Exits:\nto the south you see second room".to_string(),
    );
    message_parts.insert(MessageParts::EventText, "".to_string());
    assert_eq!(
        action_hug_subject_result,
        ParsingResult::EventSuccess(EventMessage {
            message: String::from("this narrative should replace the old one.\n\n\nExits:\nto the south you see second room"),
            templated_words: vec![],
            message_parts,
        })
    );

    // We moved south so we should receive an event message with the new room narrative
    let (new_state, move_south_result) =
        handle_movement(&new_state, Action::parse(&state, "south").movement).unwrap();
    let mut message_parts = HashMap::new();
    message_parts.insert(
        MessageParts::RoomText,
        "this is a templated which exists in the game item3.\n\nthis is a templated subject that exists in the game subject2.".to_string(),
    );
    message_parts.insert(
        MessageParts::Exits,
        "Exits:\nto the north you see first room".to_string(),
    );
    message_parts.insert(MessageParts::EventText, "".to_string());
    assert_eq!(
        move_south_result,
        ParsingResult::EventSuccess(EventMessage {
            message: String::from("this is a templated which exists in the game item3.\n\nthis is a templated subject that exists in the game subject2.\n\n\nExits:\nto the north you see first room"),
            templated_words: vec!["item3".to_string(), "subject2".to_string()],
            message_parts,
        })
    );
    assert_eq!(new_state.current_room, 2);

    // room 2 has subject2, so we should be able to talk to it. This event should
    // return you to room one, and the message to be returned should be the one
    // for that room. The narrative for the event should be appended to the room
    // narrative, or replace it if the event requires that.
    let (new_state, action_talk_subject2_result) =
        handle_event(&new_state, action_talk_subject2).unwrap();
    let mut message_parts = HashMap::new();
    message_parts.insert(
        MessageParts::RoomText,
        "this is a templated which exists in the game item3.\n\nthis is a templated subject that exists in the game subject2.\n\nthis narrative should be returned along with the text of room 1.".to_string(),
    );
    message_parts.insert(
        MessageParts::Exits,
        "Exits:\nto the north you see first room".to_string(),
    );
    message_parts.insert(
        MessageParts::EventText,
        "\nYou now have a item2".to_string(),
    );
    assert_eq!(
        action_talk_subject2_result,
        ParsingResult::EventSuccess(EventMessage {
            message: String::from("this is a templated which exists in the game item3.\n\nthis is a templated subject that exists in the game subject2.\n\nthis narrative should be returned along with the text of room 1.\n\nYou now have a item2\n\nExits:\nto the north you see first room"),
            templated_words: vec!["item3".to_string(), "subject2".to_string()],
            message_parts,
        })
    );
    // event gives item2 to player so inventory should be updated
    assert!(new_state.player.inventory.items.len() == 1);
    assert_eq!(
        &new_state.player.inventory.items[0].name, "item2",
        "item2 should be in inventory",
    );
    assert!(new_state.current_room == 1);

    //Since we left the first room and came back to it via the event,
    //we should not be able to move north as room 1 only has one exit
    //to the south
    let move_north_result =
        handle_movement(&new_state, Action::parse(&new_state, "north").movement);
    assert_eq!(
        move_north_result.unwrap_err().to_string(),
        InvalidMovement.to_string()
    );
    assert_eq!(new_state.current_room, 1);

    let (new_state, give_item_result) = handle_event(&new_state, action_give_item).unwrap();
    let mut message_parts = HashMap::new();
    message_parts.insert(
        MessageParts::EventText,
        "\nYou no longer have a item2".to_string(),
    );
    message_parts.insert(
        MessageParts::Exits,
        "Exits:\nto the south you see second room".to_string(),
    );
    message_parts.insert(
        MessageParts::RoomText,
        "text\n\nthis narrative should be returned along with the text of room 1.".to_string(),
    );
    assert_eq!(give_item_result, ParsingResult::EventSuccess(EventMessage {
        message: String::from("text\n\nthis narrative should be returned along with the text of room 1.\n\nYou no longer have a item2\n\nExits:\nto the south you see second room"),
        templated_words: vec![],
        message_parts,
    }));
    assert_eq!(new_state.player.inventory.items.len(), 0);
}

#[test]
fn it_process_action() {
    let config = Config::from_path("fixtures/");
    let state = State::init(config);
    let action_talk_subject = Action::parse(&state, "talk subject1");
    let action_give_subject = Action::parse(&state, "give subject1");
    let action_hug_subject = Action::parse(&state, "hug subject1");
    let action_talk_subject2 = Action::parse(&state, "talk subject2");
    let action_give_item = Action::parse(&state, "give item2 subject1");
    assert_eq!(action_talk_subject.action_type(), ActionType::VerbSubject);
    assert_eq!(action_give_subject.action_type(), ActionType::VerbSubject);
    assert_eq!(action_hug_subject.action_type(), ActionType::VerbSubject);
    assert_eq!(action_talk_subject2.action_type(), ActionType::VerbSubject);
    assert_eq!(action_give_item.action_type(), ActionType::VerbItemSubject);
    assert!(action_talk_subject.is_valid());
    assert!(action_give_subject.is_valid());
    assert!(action_hug_subject.is_valid());
    assert!(action_talk_subject2.is_valid());
    assert!(action_give_item.is_valid());
    assert!(action_give_subject.is_valid());
    assert_eq!(format!("{}", action_talk_subject.verb.unwrap()), "talk");
    assert_eq!(format!("{}", action_give_subject.verb.unwrap()), "give");
    assert_eq!(format!("{}", action_hug_subject.verb.unwrap()), "hug");
    assert_eq!(format!("{}", action_talk_subject2.verb.unwrap()), "talk");
    assert_eq!(format!("{}", action_give_item.verb.unwrap()), "give");
    assert_eq!(
        format!("{}", action_talk_subject.subject.unwrap()),
        "subject1",
    );
    assert_eq!(
        format!("{}", action_give_subject.subject.unwrap()),
        "subject1",
    );
    assert_eq!(
        format!("{}", action_hug_subject.subject.unwrap()),
        "subject1",
    );
    assert_eq!(
        format!("{}", action_talk_subject2.subject.unwrap()),
        "subject2",
    );
    assert_eq!(format!("{}", action_give_item.subject.unwrap()), "subject1");
}
