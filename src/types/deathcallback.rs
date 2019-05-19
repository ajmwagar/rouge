use super::*;
use crate::func::combat::{monster_death, player_death};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
/// Possible deathcallbacks
pub enum DeathCallback {
    Player,
    Monster,
}

impl DeathCallback {
    /// Fire the death callback
    pub fn callback(self, object: &mut Object, game: &mut Game) {
        let callback: fn(&mut Object, &mut Messages) = match self {
            DeathCallback::Player => player_death,
            DeathCallback::Monster => monster_death,
        };
        callback(object, &mut game.log);
    }
}
