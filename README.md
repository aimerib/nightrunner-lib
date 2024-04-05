# nightrunner-lib

This library is a text-adventure game engine that can be used to create
text based adventure games. It is designed to be used with a front-end
which can be written in any language. Implementing this library in a
language is a matter of writing a front-end an passing string data to
the library for parsing.

## Using the Rust library

The `parse_input` and `parse_input_json` functions are the only
functions that need to be called by the front-end, but the library
exposes some of the internal structs and functions to help developers
understand how the library works, and to allow a little bit of flexibility
in how the library is used.

To initialize the parser you must pass either a JSON string or a path
to YAML files containing the configuration for the game to
`NightRunnerBuilder` using the builder pattern.

### Example:

```rust
use nightrunner_lib::NightRunner;
use nightrunner_lib::NightRunnerBuilder;
use nightrunner_lib::parser::interpreter::{ParsingResult};
let nr = NightRunnerBuilder::new().with_path("/game_config/").build();
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

To run a rust example run
```shell
cargo run --example cursive_example
```

This example should give you a good idea on how to consume the library in rust, and how to structure you front-end for your game.

## Using the Wasm library

Add the nightrunner_lib package from npm to your repository:

```shell
yarn add @nightrunner/nightrunner_lib
```

---

**NOTE**

You will need a bundler to use this package. Currently I recommend using
Vite. For examples on how to use Vite with this library check out the
`examples/wasm` folder in the repository. Another popular and well supported
bundler is webpack.

---

The `parse` function is the only function that is necessary to be called by
the front-end. It receives a simple string input and returns the result as
a JSON parsed string.

To initialize the parser in wasm you must pass a JSON string with the configuration
of the game when creating a new instance of the `NightRunner` class in JavaScript.

### Example:

```ts
// This data can also be retrieved from an api endpoint with the browser
// fetch API.
import data from "./data.json";
import { NightRunner } from "@nightrunner/nightrunner_lib";

// Load the NightRunner library.
// The NightRunner class expects stringified JSON data.
const engine: NightRunner = new NightRunner(JSON.stringify(data));
let result = engine.parse("look");
// {"messageType":"look","data":"first room\n\nHere you see: \nan item1\nan item2\nsubject1"}
```
