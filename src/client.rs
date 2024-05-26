use std::collections::HashMap;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::errors::MihomoError::{HttpRequestError, UserNotFound, InvalidParams};
use crate::tools;
use crate::models::base::StarrailInfoParsed;

#[derive(Debug)]
pub enum Language {
    CHT,
    CHS,
    DE,
    EN,
    ES,
    FR,
    ID,
    JP,
    KR,
    PT,
    RU,
    TH,
    VI,
}

impl Language {
    pub fn value(&self) -> &'static str {
        match self {
            Language::CHT => "cht",
            Language::CHS => "cn",
            Language::DE => "de",
            Language::EN => "en",
            Language::ES => "es",
            Language::FR => "fr",
            Language::ID => "id",
            Language::JP => "jp",
            Language::KR => "kr",
            Language::PT => "pt",
            Language::RU => "ru",
            Language::TH => "th",
            Language::VI => "vi",
        }
    }
}

#[derive(Debug)]
pub struct MihomoAPI {
    pub base_url: String,
    pub asset_url: String,
    pub lang: Language,
    pub client: Client,
}

impl MihomoAPI {
    pub fn new(lang: Option<Language>) -> MihomoAPI {
        MihomoAPI {
            base_url: "https://api.mihomo.me/sr_info_parsed".to_string(),
            asset_url: "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master".to_string(),
            lang: lang.unwrap_or(Language::ID),
            client: Client::new(),  
        }
    }
    
    
    pub async fn request() -> () {
        todo!();
    }
    
    pub async fn fetch_user() -> () {
        todo!();
    }
    
    pub async fn get_icon_url() -> () {
        todo!()
    }
}