use std::{
    io::{BufRead, BufReader, Read},
    net::TcpStream,
    str::FromStr,
    time::Duration,
};

use super::http_client::{HttpBodyType, HttpResponse, HttpStatus};

const TRANSFER_ENCODING_CHUNKED_HEADER: &str = "transfer-encoding: chunked";
const CONTENT_LENGTH_HEADER: &str = "content-length";

pub fn parse_response(tcp_stream: TcpStream) -> HttpResponse {
    tcp_stream
        .set_read_timeout(Some(Duration::from_secs(30)))
        .unwrap();

    let mut reader = BufReader::new(&tcp_stream);
    let mut start_line = String::new();

    reader
        .read_line(&mut start_line)
        .unwrap_or_else(|e| panic!("Couldn't parse response. {e}"));

    let mut start_line_iter = start_line.split_whitespace();

    start_line_iter.next().unwrap_or("");
    let status = start_line_iter
        .next()
        .unwrap_or_else(|| panic!("Couldn't parse response status code"));

    let status = HttpStatus::from_str(status)
        .unwrap_or_else(|e| panic!("Couldn't parse response status code. {e}"));

    let headers = read_headers(&mut reader);
    let body = parse_body(headers, &mut reader);

    HttpResponse { status, body }
}

fn parse_body(headers: Vec<String>, reader: &mut BufReader<&TcpStream>) -> HttpBodyType {
    let body_header = headers
        .into_iter()
        .find(|h| {
            h.find(TRANSFER_ENCODING_CHUNKED_HEADER).is_some()
                || h.find(CONTENT_LENGTH_HEADER).is_some()
        })
        .unwrap_or("".to_string());

    let body = if body_header.find(TRANSFER_ENCODING_CHUNKED_HEADER).is_some() {
        // parse chunks
        HttpBodyType::Chunked(read_chunked_body(reader))
    } else if body_header.find(CONTENT_LENGTH_HEADER).is_some() {
        // parse body using Content-length header
        todo!()
    } else {
        // read until EOF
        let mut buffer = Vec::new();
        reader
            .read_to_end(&mut buffer)
            .unwrap_or_else(|e| panic!("Error while reading body. {e}"));

        HttpBodyType::Regular(String::from_utf8(buffer).unwrap())
    };

    body
}

fn read_headers(reader: &mut BufReader<&TcpStream>) -> Vec<String> {
    let mut buffer = Vec::new();
    let mut current_string = String::new();

    loop {
        current_string.clear();
        reader
            .read_line(&mut current_string)
            .unwrap_or_else(|e| panic!("Couldn't parse response headers. {e}"));

        if current_string == "\r\n" {
            break;
        }

        buffer.push(current_string.to_lowercase());
    }

    buffer
}

fn read_chunked_body(reader: &mut BufReader<&TcpStream>) -> Vec<String> {
    let mut buffer = Vec::new();
    let mut bytes_num = 0;
    let mut chunk = String::new();
    let mut junk = String::new();

    loop {
        chunk.clear();

        reader
            .read_line(&mut chunk)
            .unwrap_or_else(|e| panic!("Error while reading response body. {e}"));

        chunk = chunk.replace("\r\n", "");

        let chunk_length = usize::from_str_radix(&chunk, 16)
            .unwrap_or_else(|e| panic!("Couldn't parse response chunk size. {e}"));

        if chunk_length == 0 {
            break;
        }

        chunk.clear();

        loop {
            if bytes_num == chunk_length {
                reader
                    .read_line(&mut junk)
                    .unwrap_or_else(|e| panic!("Couldn't read new line character. {e}"));

                bytes_num = 0;
                buffer.push(chunk.clone());

                break;
            }

            bytes_num += reader
                .read_line(&mut chunk)
                .unwrap_or_else(|e| panic!("Error while reading response body. {e}"));
        }
    }

    buffer
}
