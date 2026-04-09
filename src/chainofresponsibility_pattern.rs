
//Request and Response types
#[derive(Debug, Clone)]
pub struct Request {
    path:        String,
    method:      String,
    auth_token:  Option<String>,
    client_ip:   String,
    body:        Option<String>,
}

impl Request {
    pub fn new(method: &str, path: &str) -> Self {
        Self {
            path:       path.to_string(),
            method:     method.to_string(),
            auth_token: None,
            client_ip:  String::from("127.0.0.1"),
            body:       None,
        }
    }

    pub fn with_token(mut self, token: &str) -> Self {
        self.auth_token = Some(token.to_string());
        self
    }

    pub fn with_ip(mut self, ip: &str) -> Self {
        self.client_ip = ip.to_string();
        self
    }
}

#[derive(Debug)]
pub struct Response {
   pub status:  u16,
   pub  body:    String,
}

impl Response {
    fn ok(body: &str) -> Self {
        Self { status: 200, body: body.to_string() }
    }
    fn unauthorized(msg: &str) -> Self {
        Self { status: 401, body: msg.to_string() }
    }
    fn too_many_requests() -> Self {
        Self { status: 429, body: String::from("Rate limit exceeded") }
    }
}

//the handler trait - the chain link contract
enum ChainResult {
    Handled(Response),   // stop — I produced a response
    Forward,             // pass to next handler
    Rejected(Response),  // stop — blocked with error response
}

pub trait Middleware {
    fn handle(&self, req: &Request) -> ChainResult;
    fn name(&self) -> &str;
}

//Handler 1: Auth middleware
pub struct AuthMiddleware {
    valid_tokens: Vec<String>,
}

impl AuthMiddleware {
   pub fn new(tokens: Vec<&str>) -> Self {
        Self {
            valid_tokens: tokens.iter().map(|t| t.to_string()).collect(),
        }
    }
}

impl Middleware for AuthMiddleware {
    fn handle(&self, req: &Request) -> ChainResult {
        match &req.auth_token {
            None => {
                println!("  [Auth] No token — rejecting");
                ChainResult::Rejected(Response::unauthorized("Missing auth token"))
            }
            Some(token) if !self.valid_tokens.contains(token) => {
                println!("  [Auth] Invalid token '{}' — rejecting", token);
                ChainResult::Rejected(Response::unauthorized("Invalid auth token"))
            }
            Some(token) => {
                println!("  [Auth] Token '{}' valid — forwarding", token);
                ChainResult::Forward
            }
        }
    }

    fn name(&self) -> &str { "Auth" }
}

//handler 2 rate limit middleware
use std::collections::HashMap;
use std::cell::RefCell;

pub struct RateLimitMiddleware {
    max_requests:   u32,
    request_counts: RefCell<HashMap<String, u32>>,
}

impl RateLimitMiddleware {
   pub fn new(max_requests: u32) -> Self {
        Self {
            max_requests,
            request_counts: RefCell::new(HashMap::new()),
        }
    }
}

impl Middleware for RateLimitMiddleware {
    fn handle(&self, req: &Request) -> ChainResult {
        let mut counts = self.request_counts.borrow_mut();
        let count = counts.entry(req.client_ip.clone()).or_insert(0);
        *count += 1;

        if *count > self.max_requests {
            println!(
                "  [RateLimit] IP {} exceeded limit ({}/{}) — rejecting",
                req.client_ip, count, self.max_requests
            );
            ChainResult::Rejected(Response::too_many_requests())
        } else {
            println!(
                "  [RateLimit] IP {} ok ({}/{}) — forwarding",
                req.client_ip, count, self.max_requests
            );
            ChainResult::Forward
        }
    }

    fn name(&self) -> &str { "RateLimit" }
}

//handler 3 logging middleware always forward , never rejects
pub struct LoggingMiddleware;

impl Middleware for LoggingMiddleware {
    fn handle(&self, req: &Request) -> ChainResult {
        println!(
            "  [Logger] {} {} from {} token={:?}",
            req.method, req.path, req.client_ip,
            req.auth_token.as_deref().unwrap_or("none")
        );
        ChainResult::Forward   // logging never blocks
    }

    fn name(&self) -> &str { "Logger" }
}

//handler 4 route handler 
pub struct RouteHandler {
    routes: HashMap<String, String>,
}

impl RouteHandler {
    pub fn new() -> Self {
        let mut routes = HashMap::new();
        routes.insert(String::from("GET /users"),    String::from(r#"{"users": ["Alice", "Bob"]}"#));
        routes.insert(String::from("GET /health"),   String::from(r#"{"status": "ok"}"#));
        routes.insert(String::from("POST /message"), String::from(r#"{"sent": true}"#));
        Self { routes }
    }
}
impl Middleware for RouteHandler {
    fn handle(&self, req: &Request) -> ChainResult {
        let key = format!("{} {}", req.method, req.path);
        match self.routes.get(&key) {
            Some(body) => {
                println!("  [Router] Matched route '{}' — handled", key);
                ChainResult::Handled(Response::ok(body))
            }
            None => {
                println!("  [Router] No route for '{}' — handled with 404", key);
                ChainResult::Handled(Response {
                    status: 404,
                    body:   format!("Route '{}' not found", key),
                })
            }
        }
    }

    fn name(&self) -> &str { "Router" }
}

//pipeline - the chain runner
pub struct Pipeline {
    handlers: Vec<Box<dyn Middleware>>,
}

impl Pipeline {
    pub fn new() -> Self {
        Self { handlers: vec![] }
    }

    // Builder-style — chain calls naturally
    pub fn add(mut self, handler: Box<dyn Middleware>) -> Self {
        self.handlers.push(handler);
        self
    }

    pub fn run(&self, req: &Request) -> Response {
        println!("\n>> {} {} (ip={})",
            req.method, req.path, req.client_ip);

        for handler in &self.handlers {
            match handler.handle(req) {
                ChainResult::Handled(resp) => {
                    println!("  Chain ended at '{}' -> {} OK",
                        handler.name(), resp.status);
                    return resp;
                }
                ChainResult::Rejected(resp) => {
                    println!("  Chain rejected at '{}' -> {}",
                        handler.name(), resp.status);
                    return resp;
                }
                ChainResult::Forward => {
                    // continue to next handler
                }
            }
        }

        // Fell off the end with no handler — shouldn't happen with a route handler last
        Response { status: 500, body: String::from("No handler matched") }
    }
}
