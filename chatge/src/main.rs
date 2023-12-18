use axum::{routing::get, Router};
use ollama_client::ollama_client::get_ollama_response;

pub mod ollama_client;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:6080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    get_ollama_response("write 2 words in response");

    "Hello, World!"
}
