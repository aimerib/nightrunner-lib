use serde::{Deserialize, Serialize};

/// Possible directions for a movement.
/// The parser currently only supports
/// cardinal directions but will be extended
/// to support other directions such as
/// "up" or "left" in the future.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Eq)]
#[serde(rename_all = "snake_case")]
pub struct AllowedDirections {
    pub(crate) directions: Vec<String>,
}

impl AllowedDirections {
    pub(crate) fn init() -> AllowedDirections {
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
    /// Move to the east.
    East,
    #[serde(rename = "north")]
    /// Move to the north.
    North,
    #[serde(rename = "south")]
    /// Move to the south.
    South,
    #[serde(rename = "west")]
    /// Move to the west.
    West,
}
impl std::fmt::Display for Directions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Directions::North => write!(f, "north"),
            Directions::South => write!(f, "south"),
            Directions::East => write!(f, "east"),
            Directions::West => write!(f, "west"),
        }
    }
}
