use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LiteServerId {
    #[serde(rename = "@type")]
    _type: String,
    key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LiteServer {
    ip: i32,
    port: u16,
    id: LiteServerId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalConfig {
    pub liteservers: Vec<LiteServer>,
}
