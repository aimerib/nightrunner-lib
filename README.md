# nightrunner-lib

This library is a text-adventure game engine that can be used to create
text based adventure games. It is designed to be used with a front-end
which can be written in any language. Implementing this library in a
language is a matter of writing a front-end an passing string data to
the library for parsing.

The configuration of the game is done in the `Config` struct
and can be initialized both with YAML files and serialized
JSON data, so it is perfect for both web and desktop games.

The `parse_input` and `parse_input_json` functions are the only
functions that need to be called by the front-end, but the library
exposes some of the internal structs and functions to help developers
understand how the library works, and to allow a little bit of flexibility
in how the library is used.

## Example:

```rust
use nightrunner_lib::NightRunner;
use nightrunner_lib::NightRunnerBuilder;
use nightrunner_lib::parser::interpreter::{ParsingResult};
let data = r#"{
  "allowed_verbs": [
    {
      "id": 1,
      "names": [
        "quit",
        ":q",
        "q"
      ],
      "verb_function": "quit"
    },
    {
      "id": 7,
      "names": [
        "give",
        "hand"
      ],
      "verb_function": "normal"
    },
    {
      "id": 2,
      "names": [
        "help"
      ],
      "verb_function": "help"
    },
    {
      "id": 3,
      "names": [
        "look",
        "stare"
      ],
      "verb_function": "look"
    },
    {
      "id": 4,
      "names": [
        "inventory",
        "i"
      ],
      "verb_function": "inventory"
    },
    {
      "id": 6,
      "names": [
        "drop",
        "place"
      ],
      "verb_function": "drop"
    },
    {
      "id": 8,
      "names": [
        "talk",
        "chat"
      ],
      "verb_function": "talk"
    },
    {
      "id": 5,
      "names": [
        "pick",
        "take",
        "grab",
        "pi",
        "tk",
        "gr",
        "get",
        "g"
      ],
      "verb_function": "take"
    },
    {
      "id": 9,
      "names": [
        "hug"
      ],
      "verb_function": "normal"
    }
  ],
  "items": [
    {
      "id": 1,
      "name": "item1",
      "description": "item 1 description",
      "can_pick": false
    },
    {
      "id": 2,
      "name": "item2",
      "description": "item 2 description",
      "can_pick": true
    }
  ],
  "subjects": [
    {
      "id": 1,
      "name": "subject1",
      "description": "a subject description",
      "default_text": "default text"
    }
  ],
  "narratives": [
    {
      "id": 1,
      "text": "text",
      "description": "text"
    },
    {
      "id": 2,
      "text": "this is a templated which exists in the game {item1}.\n\nthis is a templated subject that exists in the game {subject1}.",
      "description": "text"
    },
    {
      "id": 3,
      "text": "this narrative should replace the old one.",
      "description": "a replaced narrative"
    }
  ],
  "events": [
    {
      "id": 1,
      "name": "text",
      "description": "text",
      "location": 1,
      "destination": null,
      "narrative": 1,
      "required_verb": 2,
      "required_subject": 1,
      "required_item": null,
      "completed": false,
      "add_item": null,
      "remove_old_narrative": false,
      "remove_item": null,
      "required_events": []
    },
    {
      "id": 2,
      "name": "text",
      "description": "text",
      "location": 1,
      "destination": null,
      "narrative": 3,
      "required_verb": 9,
      "required_subject": 1,
      "required_item": null,
      "completed": false,
      "add_item": null,
      "remove_old_narrative": true,
      "remove_item": null,
      "required_events": [
        4
      ]
    },
    {
      "id": 3,
      "name": "text",
      "description": "text",
      "location": 1,
      "destination": null,
      "narrative": 2,
      "required_verb": 2,
      "required_subject": 1,
      "required_item": null,
      "completed": false,
      "add_item": null,
      "remove_old_narrative": true,
      "remove_item": null,
      "required_events": [
        2
      ]
    },
    {
      "id": 4,
      "name": "text",
      "description": "text",
      "location": 1,
      "destination": null,
      "narrative": 1,
      "required_verb": 8,
      "required_subject": 1,
      "required_item": null,
      "completed": false,
      "add_item": null,
      "remove_old_narrative": true,
      "remove_item": null,
      "required_events": []
    }
  ],
  "intro": "text",
  "rooms": [
    {
      "id": 1,
      "name": "room 1",
      "description": "first room",
      "exits": [
        {
          "room_id": 2,
          "direction": "south"
        }
      ],
      "stash": {
        "items": [],
        "item_ids": [
          1,
          2
        ]
      },
      "room_events": [
        1, 4, 2
      ],
      "narrative": 1,
      "subjects": [
        1
      ]
    },
    {
      "id": 2,
      "name": "room 2",
      "description": "second room",
      "exits": [
        {
          "room_id": 1,
          "direction": "north"
        }
      ],
      "stash": {
        "items": [],
        "item_ids": []
      },
      "room_events": [],
      "narrative": 2,
      "subjects": []
    }
  ]
}"#;
let nr = NightRunnerBuilder::new().with_json_data(data).build();
let result = nr.parse_input("look");
let json_result = nr.json_parse_input("look");
assert!(result.is_ok());
assert_eq!(result.unwrap(),
    ParsingResult::Look(String::from("first room\nHere you see: \n\na item1\na item2"))
);
assert_eq!(json_result,
    "{\"ok\":{\"look\":\"first room\\nHere you see: \\n\\na item1\\na item2\"}}".to_string()
);
```

for examples of valid YAML and JSON data, see the documentation for
the `config` module or the fixtures folder.
