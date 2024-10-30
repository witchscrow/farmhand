pub const DEFAULT_PORT: &str = "3000"; // This is stored as a string to match environment vars

/// Global Configuration for the API Server
pub struct Config {
    pub port: String,
}

impl Config {
    pub fn new() -> Self {
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| DEFAULT_PORT.to_string())
            .parse()
            .expect("PORT must be a number");

        Config { port }
    }
    /// Formats the host and port into an address for a TCPListener to bind to
    pub fn get_address(&self) -> String {
        format!("{}:{}", "0.0.0.0", &self.port)
    }
}
