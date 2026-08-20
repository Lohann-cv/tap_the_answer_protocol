use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use std::fmt;

#[derive(Debug)]
pub enum StreamType {
    Read(OwnedReadHalf),
    Write(OwnedWriteHalf),
}

#[derive(Debug, PartialEq)]
pub enum IOResult {
    Succes(String),
    Error(String),
}

#[derive(Debug, Clone)]
pub struct ClientEnvironement {
    pub is_authenticate: bool,
}

impl ClientEnvironement {
    pub fn new() -> Self {
        Self {
            is_authenticate: false,
        }
    }
}

impl fmt::Display for IOResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IOResult::Succes(message) => write!(f, "{}", message),
            IOResult::Error(message) => write!(f, "{}", message),
        }
    }
}
