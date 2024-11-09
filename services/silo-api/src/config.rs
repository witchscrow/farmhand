pub const DEFAULT_PORT: &str = "3000"; // This is stored as a string to match environment vars

/// Global Configuration for the API Server
pub struct Config {
    pub port: String,
    pub upload_dir: String,
}

impl Config {
    pub fn new() -> Self {
        // Get the port to run the app on
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| DEFAULT_PORT.to_string())
            .parse()
            .expect("PORT must be a number");

        // Get and create upload directory if it doesn't exist
        let upload_dir = match std::env::var("UPLOAD_DIR") {
            Ok(dir) => dir,
            Err(_) => "uploads".to_string(),
        };

        std::fs::create_dir_all(&upload_dir).expect("Failed to create upload directory");

        Config { port, upload_dir }
    }
    /// Formats the host and port into an address for a TCPListener to bind to
    pub fn get_address(&self) -> String {
        format!("{}:{}", "0.0.0.0", &self.port)
    }
}
