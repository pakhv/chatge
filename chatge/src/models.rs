use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct AppState {
    pub ollama_host: String,
    pub ollama_model: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub message: String,
}
