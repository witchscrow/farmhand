pub const DEFAULT_PORT: &str = "3000"; // This is stored as a string to match environment vars

/// Global Configuration for the API Server
pub struct Config {
    pub port: String,
    pub upload_dir: Option<String>,
    pub upload_bucket: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            port: Self::get_port(),
            upload_dir: Self::get_upload_dir(),
            upload_bucket: Self::get_upload_bucket(),
        }
    }
    /// Gets the port from environment variables
    pub fn get_port() -> String {
        std::env::var("PORT")
            .unwrap_or_else(|_| DEFAULT_PORT.to_string())
            .parse()
            .expect("PORT must be a number")
    }
    /// Formats the host and port into an address for a TCPListener to bind to
    pub fn get_address(&self) -> String {
        format!("{}:{}", "0.0.0.0", &self.port)
    }
    /// Gets the upload directory from environment variables and initializes it
    pub fn get_upload_dir() -> Option<String> {
        match std::env::var("STORAGE") {
            Ok(dir) => {
                std::fs::create_dir_all(&dir).expect("Failed to create upload directory");
                Some(dir)
            }
            Err(_) => None,
        }
    }
    /// Gets the Cloudflare R2 upload bucket to use from environment
    pub fn get_upload_bucket() -> Option<String> {
        std::env::var("UPLOAD_BUCKET").ok()
    }
}
