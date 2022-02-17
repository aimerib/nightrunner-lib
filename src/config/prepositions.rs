use serde::{Deserialize, Serialize};

/// The parser expects simple commands,
/// such as "go north" or "look door".
/// Instead of expecting the user to
/// figure out how to use the commands,
/// the parser will drop any determiners
/// from the user input.
///
/// For example, the input "look around"
/// will be translated as "look" for
/// parsing and return the description of
/// the current room. This is a very simplistic
/// approach to parsing, since "sneak past dragon"
/// would be translated as "sneak dragon" and
/// that has a different meaning from the original
/// intent. A possible solution for things like
/// "sneak past dragon" would be to use an event
/// requiring the verb "sneak" and the subject
/// "dragon".
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Eq)]
#[serde(rename_all = "snake_case")]
pub struct AllowedPrepositions {
    pub prepositions: Vec<String>,
}

impl AllowedPrepositions {
    pub fn init() -> AllowedPrepositions {
        let prepositions = vec![
            "aboard",
            "about",
            "above",
            "across",
            "after",
            "against",
            "along",
            "amid",
            "among",
            "around",
            "as",
            "at",
            "before",
            "behind",
            "below",
            "beneath",
            "beside",
            "between",
            "beyond",
            "but",
            "by",
            "concerning",
            "considering",
            "despite",
            "during",
            "except",
            "following",
            "for",
            "from",
            "in",
            "inside",
            "into",
            "like",
            "minus",
            "near",
            "next",
            "of",
            "off",
            "on",
            "onto",
            "opposite",
            "out",
            "outside",
            "over",
            "past",
            "per",
            "plus",
            "regarding",
            "round",
            "save",
            "since",
            "than",
            "through",
            "till",
            "to",
            "toward",
            "under",
            "underneath",
            "unlike",
            "until",
            "up",
            "upon",
            "versus",
            "via",
            "with",
            "within",
            "without",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        AllowedPrepositions { prepositions }
    }
}
