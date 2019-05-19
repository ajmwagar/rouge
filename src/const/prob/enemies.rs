//! Probability/Transition tables for various enemies within the game

use crate::types::Transition;

/*
 * Transition values (increase based on level)
 */

/// Maximum number of monsters per room
pub const MAX_MONSTERS: &[Transition] = &[
    Transition { level: 1, value: 2 },
    Transition { level: 4, value: 3 },
    Transition { level: 6, value: 5 },
];

/*
 * Troll
 */

/// Chances of a troll spawning
pub const TROLL_CHANCES: &[Transition] = &[
    Transition {
        level: 3,
        value: 15,
    },
    Transition {
        level: 5,
        value: 30,
    },
    Transition {
        level: 7,
        value: 60,
    },
    Transition {
        level: 10,
        value: 80,
    },
];

/// Base max HP of a Troll based on dungeon_level
pub const TROLL_BASE_HP: &[Transition] = &[
    Transition {
        level: 1,
        value: 40,
    },
    Transition {
        level: 2,
        value: 55,
    },
    Transition {
        level: 5,
        value: 65,
    },
    Transition {
        level: 10,
        value: 75,
    },
    Transition {
        level: 20,
        value: 150,
    },
];

/// Base power level of a troll
pub const TROLL_BASE_POW: i32 = 8;

/// Base defense level of a troll
pub const TROLL_BASE_DEF: i32 = 2;

/// Base xp level of a troll
pub const TROLL_BASE_XP: i32 = 35;

/*
 * Orc
 */

/// Base max HP of an Orc based on dungeon_level
pub const ORC_BASE_HP: &[Transition] = &[
    Transition {
        level: 1,
        value: 20,
    },
    Transition {
        level: 2,
        value: 25,
    },
    Transition {
        level: 5,
        value: 45,
    },
    Transition {
        level: 10,
        value: 50,
    },
    Transition {
        level: 20,
        value: 100,
    },
];

/// Base power of an orc
pub const ORC_BASE_POW: i32 = 4;

/// Base amount of xp from killing an orc
pub const ORC_BASE_XP: i32 = 5;

/// Amount to divide an orc's level by to calculate defense
pub const ORC_DEFENSE_OFFSET: u32 = 2;

/*
 * Weighted values
 */

/// Chances of an orc spawning
pub const ORC_WEIGHT: u32 = 80;
