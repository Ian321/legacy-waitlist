use eve_data_core::TypeID;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Character {
    pub id: i64,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corporation_id: Option<i64>,
}

#[derive(Serialize, Debug)]
pub struct Hull {
    pub id: TypeID,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WaitlistCategory {
    pub id: String,
    pub name: String,
    pub auto_detect_names: Option<Vec<String>>
}

#[derive(Serialize, Debug, Clone)]
pub struct CharacterAndLevel {
    pub id: i64,
    pub name: String,
    pub tags: Vec<String>,
    pub active_bans: Option<Vec<Ban>>,
}

#[derive(Debug, Serialize)]
pub struct Empty {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Entity {
    pub id: i64,
    pub name: Option<String>,
    pub category: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct  System {
    pub id: i64,
    pub name: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Ban {
    pub id: Option<i64>,
    pub entity: Option<Entity>,
    pub issued_at: Option<i64>,
    pub issued_by: Option<Character>,
    pub public_reason: Option<String>,
    pub reason: String,
    pub revoked_at: Option<i64>,
    pub revoked_by: Option<Character>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Alliance {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Corporation {
    pub id: i64,
    pub name: String,
    pub alliance: Option<Alliance>,
    pub last_updated: Option<i64>,
}
