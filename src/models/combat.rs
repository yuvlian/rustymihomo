use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Element {
    pub id: String,
    pub name: String,
    pub color: String,
    pub icon: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Path {
    pub id: String,
    pub name: String,
    pub icon: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trace {
    pub id: u32,
    pub name: String,
    pub level: u32,
    pub max_level: u32,
    pub element: Option<Element>,
    #[serde(rename = "type")]
    pub type_: String, 
    #[serde(rename = "type_text")]
    pub type_text: String,
    pub effect: String,
    pub effect_text: String,
    pub simple_desc: String,
    pub desc: String,
    pub icon: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TraceTreeNode {
    pub id: u32,
    pub level: u32,
    pub max_level: u32,
    pub icon: String,
    pub anchor: String,
    pub parent: Option<u32>,
}
