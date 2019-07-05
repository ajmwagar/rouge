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

/*
 * Spells
 */

/// Chances of a lightning spell spawning
pub const LIGHTNING_WEIGHTS: &[Transition] = &[
    Transition { level: 4, value: 25, }
];

/// Chances of a fireball spell spawning
pub const FIREBALL_WEIGHTS: &[Transition] = &[
    Transition { level: 6, value: 25 }
];

/// Chances of a confuse spell spawning
pub const CONFUSE_WEIGHTS: &[Transition] = &[
    Transition { level: 2, value: 10, }
];

/*
 * Equipment
 */

/// Chances of a cloth tier item spawning
pub const CLOTH_WEIGHTS: &[Transition] = &[
    Transition { level: 2, value: 10, }
];

/// Chances of a leather tier item spawning
pub const LEATHER_WEIGHTS: &[Transition] = &[
    Transition { level: 4, value: 10, }
];



/*
 * Weighted Values
 */

/// Chance of a healing potion spawning
pub const HEAL_WEIGHT: u32 = 35;
