use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Copy, Clone)]
enum HttpMethod {
    Get,
    Patch,
    Post,
    Delete,
    Put,
}

#[derive(Debug, Clone)]
struct HttpStatus(u8, &'static str);

impl Display for HttpStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{} {}", self.0, self.1)
    }
}

#[derive(Debug, Clone)]
struct HttpHeaders(HashMap<String, String>);

impl Display for HttpHeaders {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|(key, value)| { format!("{}: {}", key, value) })
                .fold(String::new(), |mut a, b| {
                    a.reserve(b.len() + 1);
                    a.push_str(&b);
                    a.push_str("\n");
                    a
                }).trim_end()
        )
    }
}

type HttpBody = String;

#[derive(Debug, Clone)]
struct HttpRequest<'r, 't> {
    method: HttpMethod,
    headers: HttpHeaders,
    body: HttpBody
}

#[derive(Debug, Clone)]
struct HttpResponse {
    status: HttpStatus,
    headers: HttpHeaders,
    body: HttpBody
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "HTTP/1.1 {}\n{}\n\n{}",
            self.status, self.headers, self.body
        )
    }
}

#[derive(Debug, Clone)]
pub struct Route {
    pub path: &'static str,
    pub method: HttpMethod,
    pub handler: fn(HttpRequest) -> HttpResponse,
}

fn handler(_: HttpRequest) -> HttpResponse {
    HttpResponse {
        status: HttpStatus(200, "OK"),
        headers: HttpHeaders(HashMap::new()),
        body: HttpBody::from("Pong"),
    }
}

static INDEX_ROUTE: Route = Route {
    path: "^/$",
    method: HttpMethod::Get,
    handler,
};

pub fn main2() {
    let resp = (INDEX_ROUTE.handler)(HttpRequest {
        method: HttpMethod::Get,
        headers: HttpHeaders(HashMap::new()),
        body: HttpBody::from("Ping")
    });
    println!("{}", resp);
}
