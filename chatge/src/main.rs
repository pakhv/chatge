use axum::{routing::post, Router};
use ollama_client::ollama_client::get_ollama_response;

pub mod ollama_client;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/chat", post(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:6080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root(message: String) -> String {
    get_ollama_response(&message)
}
