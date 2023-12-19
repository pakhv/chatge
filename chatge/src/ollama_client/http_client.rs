use std::{
    collections::HashMap,
    fmt::Display,
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
    str::FromStr,
    time::Duration,
};

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

        println!("{}", String::from_utf8(request_bytes.clone()).unwrap());

        tcp_stream.write_all(&request_bytes).unwrap_or_else(|e| {
            panic!("Error while writing to stream. {e}");
        });

        let mut buffer = String::new();
        tcp_stream
            .set_read_timeout(Some(Duration::from_secs(30)))
            .unwrap();
        let _a = tcp_stream.read_to_string(&mut buffer);

        println!("{buffer}");

        HttpResponse {
            status: HttpStatus::ServerError,
            body: None,
        }
    }
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
    pub body: Option<String>,
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
