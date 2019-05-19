use super::*;
use crate::r#const::story::*;
use crate::r#const::*;
use tcod::colors;

/// Returns a value that depends on level. the table specifies what
/// value occurs after each level, default is 0.
pub fn from_dungeon_level(table: &[Transition], level: u32) -> u32 {
    table
        .iter()
        .rev()
        .find(|transition| level >= transition.level)
        .map_or(0, |transition| transition.value)
}

/// Increase the player level and show a menu to increase stats
pub fn level_up(objects: &mut [Object], game: &mut Game, tcod: &mut Tcod) {
    let player = &mut objects[PLAYER];
    let level_up_xp = LEVEL_UP_BASE + player.level * LEVEL_UP_FACTOR;
    // see if the player's experience is enough to level-up
    if player.fighter.as_ref().map_or(0, |f| f.xp) >= level_up_xp {
        // it is! level up
        player.level += 1;
        game.log.add(
            format!("{} {}!", LEVEL_UP_MSG, player.level),
            colors::YELLOW,
        );
        let fighter = player.fighter.as_mut().unwrap();
        let mut choice = None;
        while choice.is_none() {
            // keep asking until a choice is made
            choice = menu(
                LEVEL_UP_MENU_HEADER,
                &[
                    format!("Constitution (+20 HP, from {})", fighter.base_max_hp),
                    format!("Strength (+1 attack, from {})", fighter.base_power),
                    format!("Agility (+1 defense, from {})", fighter.base_defense),
                ],
                LEVEL_SCREEN_WIDTH,
                &mut tcod.root,
            );
        }
        fighter.xp -= level_up_xp;
        match choice.unwrap() {
            0 => {
                fighter.base_max_hp += 20;
                fighter.hp = fighter.base_max_hp;
            }
            1 => {
                fighter.base_power += 1;
            }
            2 => {
                fighter.base_defense += 1;
            }
            _ => unreachable!(),
        }
    }
}

/// Advance to the next level
pub fn next_level(tcod: &mut Tcod, objects: &mut Vec<Object>, game: &mut Game) {
    game.log.add(REST_MSG, colors::VIOLET);

    // Heal 50% of the player's health
    let heal_hp = objects[PLAYER].max_hp(game) / 2;
    objects[PLAYER].heal(heal_hp, game);

    game.log.add(DUNGEON_LVL_MSG, colors::RED);
    game.dungeon_level += 1;

    objects[PLAYER].fighter.as_mut().unwrap().xp += (game.dungeon_level * LEVEL_XP_MULTIPLIER) as i32;

    game.map = make_map(objects, game.dungeon_level);
    initialise_fov(&game.map, tcod);
}
