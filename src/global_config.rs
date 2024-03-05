use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LiteServerId {
    #[serde(rename = "@type")]
    pub _type: String,
    pub key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LiteServer {
    pub ip: i32,
    pub port: u16,
    pub id: LiteServerId,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GlobalConfig {
    pub liteservers: Vec<LiteServer>,
}
