#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
/// A tile on the map
pub struct Tile {
    /// Whether the tile blocks a player's movement
    pub blocked: bool,
    /// Whether the tile blocks a player's sight (i.e. a wall)
    pub block_sight: bool,
    /// Whether the block has already been explored
    pub explored: bool,
}

impl Tile {
    /// Creates an empty tile
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            explored: false,
            block_sight: false,
        }
    }

    /// Creates a tile
    pub fn wall() -> Self {
        Tile {
            blocked: true,
            explored: false,
            block_sight: true,
        }
    }

    pub fn space() -> Self {
        Tile {
            blocked: true,
            explored: false,
            block_sight: false,
        }
    }
}
