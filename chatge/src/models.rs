use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub message: String,
}
