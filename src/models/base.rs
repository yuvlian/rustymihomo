use serde::{Deserialize, Serialize};
use crate::models::character::Character;
use crate::models::player::Player;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StarrailInfoParsed {
    pub player: Player,
    pub characters: Vec<Character>,
}
