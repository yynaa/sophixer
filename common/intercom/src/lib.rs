//! *this crate contains server boilerplate for communicating with other services*
//!
//! services concerned:
//! - Calcium
//! - Bismuth

mod server;
pub use server::Server;

use log::warn;
use std::collections::VecDeque;
use std::net::SocketAddr;
use thiserror::Error;

/// error
#[derive(Error, Debug)]
pub enum InterError {
    #[error("IO error: {0:?}")]
    IOError(std::io::Error),

    #[error("thread error: {0:?}")]
    ThreadError(String),

    #[error("mpsc sending error: {0:?}")]
    MPSCSendError(String),
}

/// trait for messages coming from clients
pub trait InterMessageIncoming: Sized {
    fn get_prefix() -> String;
    fn from_raw(raw: Vec<&str>) -> Option<Self>;
}

/// trait for message going to clients
pub trait InterMessageOutgoing: Sized {
    fn to_raw(self) -> Result<String, InterError>;
}

/// trait for a communicator
pub trait InterCommunicator<I: InterMessageIncoming, O: InterMessageOutgoing> {
    fn get_messages(server: &Server) -> Option<VecDeque<(SocketAddr, I)>> {
        server.get(I::get_prefix()).map(|deque| {
            let mut deque_clone = deque.clone();
            let mut r = VecDeque::new();
            while let Some((addr, msg_string)) = deque_clone.pop_front() {
                match I::from_raw(msg_string.split(":").collect()) {
                    None => {
                        warn!("unrecognized message from server: {msg_string:?}")
                    }
                    Some(msg) => {
                        r.push_back((addr, msg));
                    }
                }
            }
            r
        })
    }
    fn send_message(server: &Server, addr: SocketAddr, msg: O) -> Result<(), InterError> {
        let msg_string = msg.to_raw()?;
        server.send(addr, msg_string)?;
        Ok(())
    }
}
