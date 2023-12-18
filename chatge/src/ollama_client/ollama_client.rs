const OLLAMA_URL: &str = "host.docker.internal:11434";
//const CHAT_ENDPOINT: &str = "/api/chat";

use std::{
    io::{Read, Write},
    net::TcpStream,
    time::Duration,
};

pub fn get_ollama_response(user_question: &str) {
    let json_data = format!(
        r#"{{
  "model": "llama2",
  "messages": [
    {{
      "role": "user",
      "content": "write 2 words in response"
    }}
  ]
}}"#
    );
    let content_length = json_data.as_bytes().len();

    let request = format!(
        r#"POST /api/chat HTTP/1.1
Host: {OLLAMA_URL}
Content-Type: application/json
Content-length: {content_length}

{json_data}"#
    );

    let mut tcp_stream = TcpStream::connect(OLLAMA_URL).unwrap_or_else(|e| {
        panic!("Unable to connect to ollama server. {e}");
    });

    tcp_stream
        .write_all(request.as_bytes())
        .unwrap_or_else(|e| {
            panic!("Error while writing to stream. {e}");
        });

    let mut buffer = String::new();
    tcp_stream
        .set_read_timeout(Some(Duration::from_secs(30)))
        .unwrap();
    let _a = tcp_stream.read_to_string(&mut buffer);

    println!("{}", buffer);
}
