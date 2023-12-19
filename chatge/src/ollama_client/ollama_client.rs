const OLLAMA_URL: &str = "host.docker.internal:11434";
const CHAT_ENDPOINT: &str = "/api/chat";

use serde::Deserialize;

use super::http_client::{HttpMethod, HttpRequest, HttpStatus};

#[derive(Deserialize, Default)]
struct OllamaMessage {
    content: String,
}

#[derive(Deserialize)]
struct OllamaResponseChunk {
    #[serde(default)]
    message: OllamaMessage,
}

#[derive(Deserialize)]
struct OllamaDone {
    done: bool,
}

pub fn get_ollama_response(user_question: &str) -> String {
    let json_data = format!(
        r#"{{
  "model": "llama2",
  "messages": [
    {{
      "role": "user",
      "content": "{user_question}"
    }}
  ]
}}"#
    );

    let response = HttpRequest::new(OLLAMA_URL)
        .set_method(HttpMethod::Post)
        .set_endpoint(CHAT_ENDPOINT)
        .set_header("Host", OLLAMA_URL)
        .set_header("Content-Type", "application/json")
        .set_body(json_data.as_bytes())
        .send();

    match &response.status {
        HttpStatus::Ok => (),
        _ => panic!("Error while making request to ollama"),
    }

    let body = response
        .body
        .unwrap_or_else(|| panic!("Got empty body in response"));

    let mut chunks_split = body.split("\r");

    let mut response_buffer = String::new();

    while let Some(chunk) = chunks_split.next() {
        let chunk_obj: Result<OllamaResponseChunk, serde_json::Error> = serde_json::from_str(chunk);

        if chunk_obj.is_ok() {
            response_buffer.push_str(&chunk_obj.unwrap().message.content);
        } else if chunk != "" {
            let chunk_obj: OllamaDone = serde_json::from_str(chunk)
                .unwrap_or_else(|e| panic!("Failed to deserialize ollama response chunk. {e}"));

            if !chunk_obj.done {
                panic!("Something went wrong. Didn't get last ollama response chunk");
            }
        }
    }

    response_buffer
}
