use serde::{Deserialize, Serialize};

/// Possible directions for a movement.
/// The parser currently only supports
/// cardinal directions but will be extended
/// to support other directions such as
/// "up" or "left" in the future.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Eq)]
#[serde(rename_all = "snake_case")]
pub struct AllowedDirections {
    pub directions: Vec<String>,
}

impl AllowedDirections {
    pub fn init() -> AllowedDirections {
        let directions = vec![
            "north", "south", "east", "west", "up", "down", "left", "right",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        AllowedDirections { directions }
    }
}

/// Possible directions for a movement.
/// In the future this will be extended
/// so inputs like "climb down" or "go
/// left" will be supported.
#[derive(Clone, Debug, Deserialize, Serialize, Eq, Ord, PartialEq, PartialOrd)]
#[serde(rename_all = "snake_case")]
pub enum Directions {
    #[serde(rename = "east")]
    EAST,
    #[serde(rename = "north")]
    NORTH,
    #[serde(rename = "south")]
    SOUTH,
    #[serde(rename = "west")]
    WEST,
}
impl std::fmt::Display for Directions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Directions::NORTH => write!(f, "north"),
            Directions::SOUTH => write!(f, "south"),
            Directions::EAST => write!(f, "east"),
            Directions::WEST => write!(f, "west"),
        }
    }
}
