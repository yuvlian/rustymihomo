use serde::{Deserialize, Serialize};
use crate::models::stat::{Attribute, Property, MainAffix, SubAffix};
use crate::models::combat::Path;
use std::slice::IterMut;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LightCone {
    pub id: u32,
    pub name: String,
    pub rarity: u32,
    #[serde(alias = "rank")]
    pub superimpose: u32,
    pub level: u32,
    #[serde(alias = "promotion")]
    pub ascension: u32,
    pub icon: String,
    pub preview: String,
    pub portrait: String,
    pub path: Path,
    pub attributes: Vec<Attribute>, 
    pub properties: Vec<Property>, 
    pub max_level: u32, 
}

impl LightCone {
    pub fn max_level(&self) -> u32 {
        20 + 10 * self.ascension
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Relic {
    pub id: u32,
    pub name: String,
    pub set_id: u32,
    pub set_name: String,
    pub rarity: u32,
    pub level: u32,
    pub main_affix: MainAffix,
    #[serde(alias = "sub_affix")]
    pub sub_affixes: Vec<SubAffix>, 
    pub icon: String,
}

impl Relic {
    pub fn iter_mut(&mut self) -> IterMut<SubAffix> {
        self.sub_affixes.iter_mut()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RelicSet {
    pub id: u32,
    pub name: String,
    pub icon: String,
    pub num: u32,
    pub desc: String,
    pub properties: Vec<Property>, 
}