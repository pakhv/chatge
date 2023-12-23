use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use html_templates::{
    chat_page::{ChatPage, Message},
    html_template::HtmlTemplate,
};
use ollama_client::ollama_client::get_ollama_response;
use tower_http::services::ServeDir;

pub mod html_templates;
pub mod ollama_client;

#[tokio::main]
async fn main() {
    let serve_dir = ServeDir::new("static");

    let app = Router::new()
        .route("/", get(root))
        .route("/chat", post(chat))
        .route("/show-my-message", post(show_my_message))
        .nest_service("/static", serve_dir);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:6080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> impl IntoResponse {
    HtmlTemplate(ChatPage {})
}

async fn chat(message: String) -> impl IntoResponse {
    match get_ollama_response(&message) {
        Ok(result) => (StatusCode::OK, result),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err),
    }
}

async fn show_my_message(message: String) -> impl IntoResponse {
    let mut input_params = message.split('=');
    let _ = input_params.next();
    let text = input_params.next().unwrap();

    HtmlTemplate(Message {
        from: String::from("Me"),
        time: String::from("21:21"),
        text: String::from(text),
    })
}
