use tcod::map::FovAlgorithm;
use tcod::Color;

pub mod prob;
pub mod story;

/// Title of the game
pub const GAME_TITLE: &str = "Rouge";

/// Credits shown on main menu
pub const AUTHOR: &str = "By Avery Wagar (@ajmwagar)";

/// Choices on the main menu
pub const MAIN_MENU_CHOICES: &[&str] = &["Continue", "New Game", "Quit"];

/// Location of the save game from the user's home directory
pub const SAVE_GAME_LOCATION: &str = "/.cache/rouge/savegame";
// /// Save game path
// pub const SAVE_GAME_PATH: &str = "";

/// Location of the menu background
pub const MENU_BACKGROUND_PATH: &str = "./img/forest_background.png";


/*
 * Graphics
 */

/// Corpse character
pub const CORPSE: char = 1u8 as char;

/// Troll character
pub const TROLL: char = 2u8 as char;

/// Orc character
pub const ORC: char = 1u8 as char;

/// Healing potion character
pub const HEAL_POTION: char =  3u8 as char;

/// Scroll character
pub const SCROLL: char = '-';

/// Sword character
pub const SWORD: char = '/';

/// Shield Character
pub const SHIELD: char = '[';

/// Shirt Character
pub const SHIRT: char = 'S';

pub const PANTS: char = 'P';


/*
 * Walls & Floors
 */

// /// Wall character
// pub const WALL: char = 255u8 as char;

/// Vertical Wall character
pub const V_WALL: char = 186u8 as char;

/// Horizontal Wall character
pub const H_WALL: char = 205u8 as char; 
//192

/// Top Right Wall character
pub const TR_WALL: char = 187u8 as char;

/// Bottom Right Wall character
pub const BR_WALL: char = 188u8 as char;

/// Top Left Wall character
pub const TL_WALL: char = 201u8 as char;

/// Bottom Left Wall character
pub const BL_WALL: char = 200u8 as char;

// pub const T

/// Floor character
// pub const FLOOR: char = 178u8 as char; // Cobble
pub const FLOOR: char = 176u8 as char; // Gravel

/* 
 * Stats
 */

// player will always be the first object
/// Player id
pub const PLAYER: usize = 0;

// experience and level-ups
/// Level up base
pub const LEVEL_UP_BASE: i32 = 200;
/// Level up factor `base + factor(n)`
pub const LEVEL_UP_FACTOR: i32 = 150;

/// xp += dungeon_level * multiplyer 
pub const LEVEL_XP_MULTIPLIER: u32 = 10;

/*
 * UI
 */

/// Width of the main menu
pub const MAIN_MENU_WIDTH: i32 = 24;

/// Width of inventory menu
pub const INVENTORY_WIDTH: i32 = 50;
/// Width of level screen
pub const LEVEL_SCREEN_WIDTH: i32 = 40;
/// Character screen width
pub const CHARACTER_SCREEN_WIDTH: i32 = 40;


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

/*
 * Gameplay
 */

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
pub const MAP_WIDTH: i32 = SCREEN_WIDTH;
/// Map height
pub const MAP_HEIGHT: i32 = SCREEN_HEIGHT - 7;

/*
 * Colors
 */

/// RGB color of dark wall
pub const COLOR_DARK_WALL: Color = Color {
    r: 64,
    g: 64,
    b: 64,
};
// pub const COLOR_DARK_WALL: Color = Color {
//     r: 64,
//     g: 64,
//     b: 64,
// };

/// RGB color of light wall
pub const COLOR_LIGHT_WALL: Color = Color {
    r: 1,
    g: 82,
    b: 93,
};
// pub const COLOR_LIGHT_WALL: Color = Color {
//     r: 130,
//     g: 110,
//     b: 50,
// };

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
