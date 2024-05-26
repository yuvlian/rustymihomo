use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attribute {
    pub field: String,
    pub name: String,
    pub icon: String,
    pub value: f64,
    #[serde(alias = "display")]
    pub displayed_value: String,
    #[serde(alias = "percent")]
    pub is_percent: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Property {
    #[serde(rename = "type")]
    pub type_: String,
    pub field: String,
    pub name: String,
    pub icon: String,
    pub value: f64,
    #[serde(alias = "display")]
    pub displayed_value: String,
    #[serde(alias = "percent")]
    pub is_percent: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MainAffix {
    #[serde(rename = "type")]
    pub type_: String,
    pub field: String,
    pub name: String,
    pub icon: String,
    pub value: f64,
    #[serde(alias = "display")]
    pub displayed_value: String,
    #[serde(alias = "percent")]
    pub is_percent: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubAffix {
    #[serde(rename = "type")]
    pub type_: String,
    pub field: String,
    pub name: String,
    pub icon: String,
    pub value: f64,
    #[serde(alias = "display")]
    pub displayed_value: String,
    #[serde(alias = "percent")]
    pub is_percent: bool,
    
    pub count: u32,
    pub step: u32,
}