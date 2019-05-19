//! Loot tables / Stats for various objects within the game
use crate::types::Transition;

/*
 * Transition values (increase based on level)
 */

/// Maximum number of items per room
pub const MAX_ITEMS: &[Transition] = &[
    Transition { level: 1, value: 1 },
    Transition { level: 4, value: 2 },
    Transition { level: 6, value: 4 },
];

/// Chances of a lightning spell spawning
pub const LIGHTNING_WEIGHTS: &[Transition] = &[Transition {
    level: 4,
    value: 25,
}];

/// Chances of a fireball spell spawning
pub const FIREBALL_WEIGHTS: &[Transition] = &[Transition {
    level: 6,
    value: 25,
}];

/*
 * Weighted Values
 */

/// Chance of a healing potion spawning
pub const HEAL_WEIGHT: u32 = 35;
