use serde::{Deserialize, Serialize};

/// The parser expects simple commands,
/// such as "go north" or "look door".
/// Instead of expecting the user to
/// figure out how to use the commands,
/// the parser will drop any determiners
/// from the user input.
///
/// For example, the input "give my wallet"
/// will be translated as "give wallet" for
/// parsing. The same will happen for "take
/// all the money", and that will be translated
/// as "take money". This is a very simplistic
/// approach to parsing, since "take all my money"
/// would be translated as "take money" and
/// that has a different meaning from the original
/// intent.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Eq)]
#[serde(rename_all = "snake_case")]
pub struct AllowedDeterminers {
    pub(crate) determiners: Vec<String>,
}

impl AllowedDeterminers {
    pub(crate) fn init() -> AllowedDeterminers {
        let determiners = vec![
            "my",
            "our",
            "your",
            "his",
            "her",
            "its",
            "their",
            "first",
            "second",
            "third",
            "next",
            "last",
            "much",
            "some",
            "no",
            "any",
            "many",
            "enough",
            "several",
            "little",
            "all",
            "lot of",
            "plenty of",
            "another",
            "a",
            "an",
            "the",
            "each",
            "every",
            "neither",
            "either",
            "one",
            "two",
            "three",
            "ten",
            "fifty",
            "hundred",
            "thousand",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        AllowedDeterminers { determiners }
    }
}
