use serde::{Deserialize, Serialize};
use crate::models::combat::{Path, Element, Trace, TraceTreeNode};
use crate::models::equipment::{LightCone, Relic, RelicSet};
use crate::models::stat::{Attribute, Property};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    // Basic Info
    pub id: String,
    pub name: String,
    pub rarity: u32,
    pub level: u32,
    pub max_level: u32,
    #[serde(alias = "promotion")]
    pub ascension: u32,
    #[serde(alias = "rank")]
    pub eidolon: u32,
    #[serde(alias = "rank_icons")]
    pub eidolon_icons: Vec<String>,

    // Image
    pub icon: String,
    pub preview: String,
    pub portrait: String,

    // Combat
    pub path: Path,
    pub element: Element,
    #[serde(alias = "skills")]
    pub traces: Vec<Trace>,
    #[serde(alias = "skill_trees")]
    pub trace_tree: Vec<TraceTreeNode>,

    // Equipment
    pub light_cone: Option<LightCone>,
    pub relics: Option<Vec<Relic>>,
    pub relic_sets: Option<Vec<RelicSet>>,

    // Stats
    pub attributes: Vec<Attribute>,
    pub additions: Vec<Attribute>,
    pub properties: Vec<Property>,
}

impl Character {
    pub fn max_level(&self) -> u32 {
        20 + 10 * self.ascension
    }
}
