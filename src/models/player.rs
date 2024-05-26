use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Avatar {
    pub id: u32,
    pub name: String,
    pub icon: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForgottenHall {
    #[serde(alias = "level")]
    pub memory: u32,
    #[serde(alias = "chaos_id")]
    pub memory_of_chaos_id: Option<u32>,
    #[serde(alias = "chaos_level")]
    pub memory_of_chaos: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub uid: u32,
    #[serde(alias = "nickname")]
    pub name: String,
    pub level: u32,
    pub world_level: u32,
    pub friend_count: u32,
    pub avatar: Avatar,
    pub signature: String,
    pub is_display: bool,

    #[serde(alias = "memory_data")]
    pub forgotten_hall: Option<ForgottenHall>,
    #[serde(alias = "universe_level")]
    pub simulated_universes: u32,
    #[serde(alias = "light_cone_count")]
    pub light_cones: u32,
    #[serde(alias = "avatar_count")]
    pub characters: u32,
    #[serde(alias = "achievement_count")]
    pub achievements: u32,
}
