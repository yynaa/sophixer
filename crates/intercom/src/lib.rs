//! *this crate contains server boilerplate for communicating with other services*
//!
//! services concerned:
//! - Calcium
//! - Bismuth

pub mod client;
pub mod server;

use serde::{Deserialize, Serialize};
use serde_value::Value;
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

  #[error("serde JSON error: {0:?}")]
  SerdeJSONError(#[from] serde_json::Error),

  #[error("serde-value error: {0:?}")]
  SerdeValueSerializerError(#[from] serde_value::SerializerError),

  #[error("thread error: {0:?}")]
  ThreadError(String),

  #[error("mpsc sending error: {0:?}")]
  MPSCSendError(String),

  #[error("invalid socket address string: {0}")]
  NoSocketAddr(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrefixedMessage {
  prefix: String,
  message: Value,
}

#[async_trait::async_trait]
pub trait InterMessagePrefixed {
  fn get_prefix() -> String;
}

/// trait for messages coming from clients
#[async_trait::async_trait]
pub trait InterMessageIncoming<'de>: Send + Sized + Deserialize<'de> {}

/// trait for message going to clients
#[async_trait::async_trait]
pub trait InterMessageOutgoing: Send + Sized + Serialize {}
