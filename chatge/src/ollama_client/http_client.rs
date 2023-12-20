use std::{
    collections::HashMap,
    fmt::Display,
    io::{BufRead, BufReader, Write},
    net::{TcpStream, ToSocketAddrs},
    str::FromStr,
    time::Duration,
    usize,
};

const TRANSFER_ENCODING_CHUNKED_HEADER: &str = "transfer-encoding: chunked";
const CONTENT_LENGTH_HEADER: &str = "content-length";

#[derive(Debug)]
pub struct HttpRequest<'a, A>
where
    A: ToSocketAddrs,
{
    addr: A,
    headers: HashMap<String, String>,
    method: HttpMethod,
    body: Option<&'a [u8]>,
    endpoint: String,
}

impl<'a, A> HttpRequest<'a, A>
where
    A: ToSocketAddrs,
{
    pub fn new(addr: A) -> HttpRequest<'a, A> {
        HttpRequest {
            addr,
            headers: HashMap::new(),
            method: HttpMethod::Get,
            body: None,
            endpoint: "/".to_string(),
        }
    }

    pub fn set_header(mut self, name: &str, value: &str) -> Self {
        self.headers.insert(name.to_string(), value.to_string());
        self
    }

    pub fn set_method(mut self, method: HttpMethod) -> Self {
        self.method = method;
        self
    }

    pub fn set_body(mut self, body: &'a [u8]) -> Self {
        self.body = Some(body);
        self
    }

    pub fn set_endpoint(mut self, endpoint: &str) -> Self {
        self.endpoint = endpoint.to_string();
        self
    }

    pub fn send(&self) -> HttpResponse {
        let mut tcp_stream = TcpStream::connect(&self.addr).unwrap_or_else(|e| {
            panic!("Unable to connect to ollama server. {e}");
        });

        let mut request = String::new();
        let first_line = format!("{} {} HTTP/1.1\r\n", self.method, self.endpoint);
        request.push_str(&first_line);

        for (name, value) in &self.headers {
            let header_str = format!("{}: {}\r\n", name, value);
            request.push_str(&header_str);
        }

        let request_bytes = if self.body.is_some() {
            let mut body = Vec::from(self.body.unwrap());
            let content_length_header = format!("Content-length: {}\r\n\r\n", body.len());
            request.push_str(&content_length_header);

            let mut bytes = Vec::from(request.as_bytes());
            bytes.append(&mut body);

            bytes
        } else {
            todo!()
        };

        tcp_stream.write_all(&request_bytes).unwrap_or_else(|e| {
            panic!("Error while writing to stream. {e}");
        });

        parse_response(tcp_stream)
    }
}

fn parse_response(tcp_stream: TcpStream) -> HttpResponse {
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
        .iter()
        .find(|h| {
            h.find(TRANSFER_ENCODING_CHUNKED_HEADER).is_some()
                || h.find(CONTENT_LENGTH_HEADER).is_some()
        })
        .unwrap_or_else(|| {
            panic!("Couldn't find \"Content-length\" or \"Transfer-encoding: chunked\" headers")
        });

    let body = if body_header.find(TRANSFER_ENCODING_CHUNKED_HEADER).is_some() {
        HttpBodyType::Chunked(read_chunked_body(reader))
    } else {
        // parse body using Content-length header
        todo!()
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

#[derive(Debug)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpMethod::Get => todo!(),
            HttpMethod::Post => write!(f, "POST"),
            HttpMethod::Put => todo!(),
            HttpMethod::Delete => todo!(),
        }
    }
}

#[derive(Debug)]
pub enum HttpStatus {
    Ok,
    BadRequest,
    ServerError,
}

#[derive(Debug)]
pub struct HttpResponse {
    pub status: HttpStatus,
    pub body: HttpBodyType,
}

#[derive(Debug)]
pub enum HttpBodyType {
    Regular(String),
    Chunked(Vec<String>),
    None,
}

impl FromStr for HttpStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "200" => Ok(HttpStatus::Ok),
            "400" => Ok(HttpStatus::Ok),
            "500" => Ok(HttpStatus::Ok),
            _ => Err("Unknown status code response".to_string()),
        }
    }
}
