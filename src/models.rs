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

#[derive(Debug, Serialize, Deserialize)]
pub struct ButtonRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
