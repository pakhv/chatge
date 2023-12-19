const OLLAMA_URL: &str = "host.docker.internal:11434";
const CHAT_ENDPOINT: &str = "/api/chat";

use super::http_client::{HttpMethod, HttpRequest};

pub fn get_ollama_response(user_question: &str) {
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

    let request = HttpRequest::new(OLLAMA_URL)
        .set_method(HttpMethod::Post)
        .set_endpoint(CHAT_ENDPOINT)
        .set_header("Host", OLLAMA_URL)
        .set_header("Content-Type", "application/json")
        .set_body(json_data.as_bytes())
        .send();

    println!("{request:?}");
}
