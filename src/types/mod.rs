//! Types in the Rouge games
use super::*;

use tcod::colors::{self, Color};
use tcod::console::*;
use tcod::map::Map as FovMap;
use tcod::namegen::Namegen;

use tcod::input::Mouse;

// Import types
pub mod deathcallback;
pub mod item;
pub mod object;
pub mod rect;
pub mod slot;
pub mod tile;

// Export Types
pub use self::deathcallback::DeathCallback;
pub use self::item::Item;
pub use self::object::Object;
pub use self::rect::Rect;
pub use self::slot::Slot;
pub use self::tile::Tile;

// Smaller types
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
/// Enemy AI types
pub enum Ai {
    Basic,
    Confused {
        previous_ai: Box<Ai>,
        num_turns: i32,
    },
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// Possible Player actions
pub enum PlayerAction {
    TookTurn,
    DidntTakeTurn,
    Exit,
}

/// Result of using a item
pub enum UseResult {
    UsedUp,
    UsedAndKept,
    Cancelled,
}

/// A Type alias for the Map
pub type Map = Vec<Vec<Tile>>;

/// A Queue of Messages
pub type Messages = Vec<(String, Color)>;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
/// An object that can be equipped, yielding bonuses.
pub struct Equipment {
    pub slot: Slot,
    pub equipped: bool,
    pub max_hp_bonus: i32,
    pub power_bonus: i32,
    pub defense_bonus: i32,
}

/// A Weighting based on a dungeon level
pub struct Transition {
    pub level: u32,
    pub value: u32,
}

pub trait MessageLog {
    fn add<T: Into<String>>(&mut self, message: T, color: Color);
}

impl MessageLog for Messages {
    fn add<T: Into<String>>(&mut self, message: T, color: Color) {
        self.push((message.into(), color));
    }
}

#[derive(Serialize, Deserialize)]
/// Main game state
pub struct Game {
    pub map: Map,
    pub log: Messages,
    pub inventory: Vec<Object>,
    pub dungeon_level: u32,
}

// Cleaner params
/// Tcod objects
pub struct Tcod {
    /// TCOD root
    pub root: Root,
    pub con: Offscreen,
    pub panel: Offscreen,
    /// FOV map
    pub fov: FovMap,
    /// Mouse location
    pub mouse: Mouse,
    pub namegen: Namegen
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
/// A Fighter struct
/// combat-related properties and methods (monster, player, NPC).
pub struct Fighter {
    /// Current HP of the fighter
    pub hp: i32,
    /// Base Max HP
    pub base_max_hp: i32,
    /// Base Defense
    pub base_defense: i32,
    /// Base Attack power
    pub base_power: i32,
    /// What to do on death
    pub on_death: DeathCallback,
    /// XP gained for dying
    pub xp: i32,
}
