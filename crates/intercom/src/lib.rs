//! *this crate contains server boilerplate for communicating with other services*
//!
//! services concerned:
//! - Calcium
//! - Bismuth

pub mod client;
pub mod server;

use thiserror::Error;

/// error
#[derive(Error, Debug)]
pub enum InterError {
  #[error("External error: {0:?}")]
  CustomError(String),

  #[error("IO error: {0:?}")]
  IOError(std::io::Error),

  #[error("thread error: {0:?}")]
  ThreadError(String),

  #[error("mpsc sending error: {0:?}")]
  MPSCSendError(String),

  #[error("invalid socket address string: {0}")]
  NoSocketAddr(String),
}

pub trait InterMessagePrefixed {
  fn get_prefix() -> String;
}

/// trait for messages coming from clients
pub trait InterMessageIncoming: Sized {
  fn from_raw(raw: Vec<&str>) -> Option<Self>;
}

/// trait for message going to clients
pub trait InterMessageOutgoing: Sized {
  fn to_raw(self) -> Result<String, InterError>;
}
