#[derive(Debug, Clone)]
pub enum HttpMethod {
    GET,
    POST,
}
impl HttpMethod {
    pub fn from_str(s: &str) -> Self {
        match s {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            _ => HttpMethod::GET,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            HttpMethod::GET => "GET".to_string(),
            HttpMethod::POST => "POST".to_string(),
        }
    }
}
