use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PingRequest {
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PingResponse {
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MousePosition {
    pub x: f64,
    pub y: f64,
}
