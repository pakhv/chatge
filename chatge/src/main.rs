use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use html_templates::{
    chat_page::{ChatPage, Clicked},
    html_template::HtmlTemplate,
};
use ollama_client::ollama_client::get_ollama_response;

pub mod html_templates;
pub mod ollama_client;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/clicked", post(clicked))
        .route("/chat", post(chat));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:6080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> impl IntoResponse {
    HtmlTemplate(ChatPage {})
}

async fn clicked() -> impl IntoResponse {
    HtmlTemplate(Clicked {
        message: String::from("YO NERD"),
    })
}

async fn chat(message: String) -> impl IntoResponse {
    match get_ollama_response(&message) {
        Ok(result) => (StatusCode::OK, result),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err),
    }
}
