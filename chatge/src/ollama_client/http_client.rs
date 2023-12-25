use std::{
    collections::HashMap,
    fmt::Display,
    io::Write,
    net::{TcpStream, ToSocketAddrs},
    str::FromStr,
};

use super::utils::parse_response;

#[derive(Debug)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
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

    pub fn send(&self) -> Result<HttpResponse, String> {
        let mut tcp_stream =
            TcpStream::connect(&self.addr).to_result("Unable to connect to ollama server")?;

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

        tcp_stream
            .write_all(&request_bytes)
            .to_result("Error while writing to stream")?;

        Ok(parse_response(tcp_stream)?)
    }
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

impl FromStr for HttpStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "200" => Ok(HttpStatus::Ok),
            "400" => Ok(HttpStatus::BadRequest),
            "500" => Ok(HttpStatus::ServerError),
            _ => Err("Unknown status code response".to_string()),
        }
    }
}

pub trait ToResult<T> {
    fn to_result(self, message: &str) -> Result<T, String>;
}

impl<T> ToResult<T> for std::io::Result<T> {
    fn to_result(self, message: &str) -> Result<T, String> {
        match self {
            Ok(result) => Ok(result),
            Err(err) => Err(format!("{message}. {err}")),
        }
    }
}
