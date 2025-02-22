use thiserror::Error;

#[derive(Error, Debug)]
pub enum QueueError {
    #[error("Invalid Connection: {0}")]
    InvalidConnection(String),
}

#[derive(Error, Debug)]
pub enum StreamError {
    #[error("Invalid Connection: {0}")]
    InvalidConnection(String),
}
