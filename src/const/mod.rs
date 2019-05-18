use super::*;

use tcod::map::FovAlgorithm;
use tcod::Color;

pub mod story;
pub mod prob;

/// Location of the save game from the user's home directory
pub const SAVE_GAME_LOCATION: &str = "/.cache/rouge/savegame";
/// Save game path
pub const SAVE_GAME_PATH: &str = "/.cache/rouge/";

/// Corpse character
pub const CORPSE: char = 1u8 as char;
/// Troll character
pub const TROLL: char = 161u8 as char;
/// Orc character
pub const ORC: char = 160u8 as char;
/// Wall character
pub const WALL: char = 164u8 as char;
/// Floor character
pub const FLOOR: char = 178u8 as char;

/// Width of inventory menu
pub const INVENTORY_WIDTH: i32 = 50;
/// Width of level screen
pub const LEVEL_SCREEN_WIDTH: i32 = 40;
/// Character screen width
pub const CHARACTER_SCREEN_WIDTH: i32 = 40;

// player will always be the first object
/// Player id
pub const PLAYER: usize = 0;

// experience and level-ups
/// Level up base
pub const LEVEL_UP_BASE: i32 = 200;
/// Level up factor `base + factor(n)`
pub const LEVEL_UP_FACTOR: i32 = 150;

// Message Console
/// Message X offset
pub const MSG_X: i32 = BAR_WIDTH + 2;
/// Message bar width
pub const MSG_WIDTH: i32 = SCREEN_WIDTH - BAR_WIDTH - 2;
/// Message bar height
pub const MSG_HEIGHT: usize = PANEL_HEIGHT as usize - 1;

// sizes and coordinates relevant for the GUI
/// status bar width
pub const BAR_WIDTH: i32 = 20;
/// Panel height
pub const PANEL_HEIGHT: i32 = 7;
/// Panel y-offset
pub const PANEL_Y: i32 = SCREEN_HEIGHT - PANEL_HEIGHT;

// Screen Size
/// Screen width
pub const SCREEN_WIDTH: i32 = 80;
/// Screen height
pub const SCREEN_HEIGHT: i32 = 40;
/// Max FPS
pub const LIMIT_FPS: i32 = 60;

// Spell constants
/// Amount of heal restored by healing potion
pub const HEAL_AMOUNT: i32 = 40;
/// Amount of base damage done by lightning
pub const LIGHTNING_DAMAGE: i32 = 40;
/// Max range of lightning spell
pub const LIGHTNING_RANGE: i32 = 20;
/// Amount of base damage done by fireball
pub const FIREBALL_DAMAGE: i32 = 25;
/// Size of fireball
pub const FIREBALL_RADIUS: i32 = 3;
/// Amount of turns confuse potion lasts
pub const CONFUSE_NUM_TURNS: i32 = 5;
/// Max range of confuse potion
pub const CONFUSE_RANGE: i32 = 20;

// Room Generation
/// Maximum size of a room (square)
pub const ROOM_MAX_SIZE: i32 = 10;
/// Minimum size of a room (square)
pub const ROOM_MIN_SIZE: i32 = 6;
/// Maximum rooms per level
pub const MAX_ROOMS: i32 = 30;

// Map
/// Map width
pub const MAP_WIDTH: i32 = 80;
/// Map height
pub const MAP_HEIGHT: i32 = 33;

/// RGB color of dark wall
pub const COLOR_DARK_WALL: Color = Color {
    r: 64,
    g: 64,
    b: 64,
};

/// RGB color of light wall
pub const COLOR_LIGHT_WALL: Color = Color {
    r: 130,
    g: 110,
    b: 50,
};

/// RGB color of dark ground
pub const COLOR_DARK_GROUND: Color = Color {
    r: 96,
    g: 96,
    b: 96,
};

/// RGB color of light ground
pub const COLOR_LIGHT_GROUND: Color = Color {
    r: 200,
    g: 180,
    b: 50,
};

/// Default FOV algorithm
pub const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
/// Should FOV use light walls
pub const FOV_LIGHT_WALLS: bool = true;
/// FOV torch radius
pub const TORCH_RADIUS: i32 = 15;
