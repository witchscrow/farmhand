pub mod s3;

/// Get the directory for where to store videos
pub fn get_storage_dir() -> String {
    let storage_dir = std::env::var("STORAGE").unwrap_or_else(|_| "storage".to_string());
    std::fs::create_dir_all(&storage_dir).unwrap();
    storage_dir
}
