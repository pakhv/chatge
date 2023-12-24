use serde::Deserialize;

use super::http_client::{HttpBodyType, HttpMethod, HttpRequest, HttpStatus};

#[derive(Deserialize, Default)]
struct OllamaMessage {
    content: String,
}

#[derive(Deserialize)]
struct OllamaResponseChunk {
    #[serde(default)]
    message: OllamaMessage,
}
const CHAT_ENDPOINT: &str = "/api/chat";

pub fn get_ollama_response(
    user_question: &str,
    ollama_url: String,
    ollama_model: String,
) -> Result<String, String> {
    let json_data = format!(
        r#"{{
  "model": "{ollama_model}",
  "messages": [
    {{
      "role": "user",
      "content": "{user_question}"
    }}
  ]
}}"#
    );

    let response = HttpRequest::new(ollama_url.clone())
        .set_method(HttpMethod::Post)
        .set_endpoint(CHAT_ENDPOINT)
        .set_header("Host", &ollama_url)
        .set_header("Content-Type", "application/json")
        .set_body(json_data.as_bytes())
        .send();

    match &response.status {
        HttpStatus::Ok => (),
        _ => {
            if let HttpBodyType::Regular(body) = response.body {
                return Err(body);
            }

            return Err(String::from("Surely it will not get here"));
        }
    }

    let mut response_buffer = String::new();

    match response.body {
        super::http_client::HttpBodyType::Chunked(chunks) => {
            for chunk in chunks {
                let chunk_obj: OllamaResponseChunk = serde_json::from_str(&chunk)
                    .unwrap_or_else(|e| panic!("Failed to deserialize ollama response chunk. {e}"));

                response_buffer.push_str(&chunk_obj.message.content);
            }
        }
        _ => todo!(),
    }

    Ok(response_buffer)
}
