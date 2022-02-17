use serde::{Deserialize, Serialize};

/// Allowed movement verbs. This is used to
/// determine if a movement is valid when
/// parsing the input.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Eq)]
#[serde(rename_all = "snake_case")]
pub struct AllowedMovements {
    pub movements: Vec<String>,
}

impl AllowedMovements {
    pub fn init() -> AllowedMovements {
        let movements = vec![
            "go", "move", "run", "walk", "jog", "amble", "dart", "limp", "saunter", "scamper",
            "scurry", "stagger", "strut", "swagger", "tiptoe", "waltz", "sneak",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        AllowedMovements { movements }
    }
}
