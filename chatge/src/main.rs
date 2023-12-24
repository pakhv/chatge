use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use chrono::{DateTime, Local};
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
        .route("/get-bot-response", post(get_bot_response))
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
    let text = input_params.next().unwrap().replace("%20", " ");

    let datetime: DateTime<Local> = Local::now();
    let time = datetime.format("%H:%M").to_string();

    HtmlTemplate(Message {
        from: String::from("Me"),
        time,
        text,
    })
}

async fn get_bot_response(message: String) -> impl IntoResponse {
    let mut input_params = message.split('=');
    let _ = input_params.next();
    let text = input_params.next().unwrap().replace("%20", " ");

    match get_ollama_response(&text) {
        Ok(result) => {
            let datetime: DateTime<Local> = Local::now();
            let time = datetime.format("%H:%M").to_string();

            return HtmlTemplate(Message {
                from: String::from("Bot"),
                time,
                text: result,
            })
            .into();
        }
        Err(err) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(format!("{err}")))
            .unwrap(),
    }
}
