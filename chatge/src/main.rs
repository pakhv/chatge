use std::env;

use axum::{
    body::Body,
    extract::State,
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
use models::{AppState, ChatRequest};
use ollama_client::ollama_client::get_ollama_response;
use tower_http::services::ServeDir;

pub mod html_templates;
pub mod models;
pub mod ollama_client;

const CHATGE_OLLAMA_HOST: &str = "CHATGE_OLLAMA_HOST";
const CHATGE_OLLAMA_MODEL: &str = "CHATGE_OLLAMA_MODEL";

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    let state = AppState {
        ollama_host: env::var(CHATGE_OLLAMA_HOST).unwrap_or_else(|_| {
            panic!("Couldn't find environment variable \"{CHATGE_OLLAMA_HOST}\"")
        }),
        ollama_model: env::var(CHATGE_OLLAMA_MODEL).unwrap_or_else(|_| {
            panic!("Couldn't find environment variable \"{CHATGE_OLLAMA_MODEL}\"")
        }),
    };

    let serve_dir = ServeDir::new("static");

    let app = Router::new()
        .route("/", get(root))
        .route("/show-my-message", post(show_my_message))
        .route("/get-bot-response", post(get_bot_response))
        .nest_service("/static", serve_dir)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:6080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> impl IntoResponse {
    HtmlTemplate(ChatPage {})
}

async fn show_my_message(request: String) -> impl IntoResponse {
    let request: ChatRequest = serde_json::from_str(&request).unwrap();

    let datetime: DateTime<Local> = Local::now();
    let time = datetime.format("%H:%M").to_string();

    HtmlTemplate(Message {
        from: String::from("Me"),
        time,
        text: request.message,
    })
}

async fn get_bot_response(State(state): State<AppState>, request: String) -> impl IntoResponse {
    let handle = tokio::spawn(async move {
        let request: ChatRequest = serde_json::from_str(&request).unwrap();

        match get_ollama_response(&request.message, state.ollama_host, state.ollama_model) {
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
    });

    handle.await.unwrap()
}
