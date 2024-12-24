pub const DEFAULT_PORT: &str = "3000"; // This is stored as a string to match environment vars

/// Global Configuration for the API Server
pub struct Config {
    pub port: String,
    pub upload_dir: Option<String>,
    pub s3_options: Option<S3Config>,
}

/// S3 compatible configuration options
pub struct S3Config {
    endpoint: String,
    bucket: String,
    access_key: String,
    secret_key: String,
    region: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            port: Self::get_port(),
            upload_dir: Self::get_upload_dir(),
            s3_options: Self::get_s3_config(),
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
        match std::env::var("UPLOAD_DIR") {
            Ok(dir) => {
                std::fs::create_dir_all(&dir).expect("Failed to create upload directory");
                Some(dir)
            }
            Err(_) => None,
        }
    }
    /// Gets the S3 configuration from environment variables
    pub fn get_s3_config() -> Option<S3Config> {
        let endpoint = std::env::var("R2_ENDPOINT");
        let bucket = std::env::var("R2_BUCKET");
        let access_key = std::env::var("R2_ACCESS_KEY_ID");
        let secret_key = std::env::var("R2_SECRET_ACCESS_KEY");
        let region = std::env::var("R2_REGION");

        let config_values = [&endpoint, &bucket, &access_key, &secret_key, &region];
        // Check if any of the variables exist or if they're all empty
        let has_some = config_values.iter().any(|result| result.is_ok());
        let has_all = config_values.iter().all(|result| result.is_ok());

        // Warn the user that the configuration was ignored due to missing values
        if has_some && !has_all {
            tracing::warn!(
                "Some S3 configuration variables are set but not all - S3 storage will be disabled"
            );
            return None;
        }
        // Just return none if nothing was specified
        if !has_some {
            return None;
        }

        Some(S3Config {
            endpoint: endpoint.unwrap(),
            bucket: bucket.unwrap(),
            access_key: access_key.unwrap(),
            secret_key: secret_key.unwrap(),
            region: region.unwrap_or_else(|_| "auto".to_string()),
        })
    }
}
