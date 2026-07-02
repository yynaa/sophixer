//! *this crate contains server boilerplate for communicating with other services*
//!
//! services concerned:
//! - Calcium
//! - Bismuth

pub mod client;
pub mod server;

use thiserror::Error;

#[macro_use]
extern crate log;

/// error
#[derive(Error, Debug)]
pub enum InterError {
  #[error("External error: {0:?}")]
  CustomError(String),

  #[error("IO error: {0:?}")]
  IOError(#[from] std::io::Error),

  #[error("thread error: {0:?}")]
  ThreadError(String),

  #[error("mpsc sending error: {0:?}")]
  MPSCSendError(String),

  #[error("invalid socket address string: {0}")]
  NoSocketAddr(String),

  #[error("empty message")]
  EmptyMessage,

  #[error("failed to serialize")]
  NoSerialization,
}

#[async_trait::async_trait]
pub trait InterMessagePrefixed {
  fn get_prefix() -> u8;
}

pub(crate) fn extract_prefix(mut bytes: Vec<u8>) -> Result<(u8, Vec<u8>), InterError> {
  if bytes.len() == 0 {
    Err(InterError::EmptyMessage)
  } else {
    let prefix = bytes.remove(0);
    Ok((prefix, bytes))
  }
}

/// trait for messages coming from clients
#[async_trait::async_trait]
pub trait InterMessageIncoming: Send + Sized + std::fmt::Debug {
  fn deserialize(bytes: Vec<u8>) -> Option<Self>;
}

/// trait for message going to clients
#[async_trait::async_trait]
pub trait InterMessageOutgoing: Send + Sized + std::fmt::Debug {
  fn serialize(&self) -> Option<Vec<u8>>;
}
