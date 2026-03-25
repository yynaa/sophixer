pub mod udp;

use log::warn;
use std::{collections::VecDeque, net::SocketAddr};

use crate::{InterError, InterMessageIncoming, InterMessageOutgoing, InterMessagePrefixed};

pub trait InterServer: Sized {
  fn start(addr: &str) -> Result<Self, InterError>;
  fn stop(self) -> Result<(), InterError>;
  fn send(&self, addr: SocketAddr, msg: String) -> Result<(), InterError>;
  fn fetch(&mut self) -> Result<(), InterError>;
  fn get(&self, prefix: String) -> Option<&VecDeque<(SocketAddr, String)>>;
}

pub trait InterServerCommunicator<
  S: InterServer,
  I: InterMessageIncoming + InterMessagePrefixed,
  O: InterMessageOutgoing,
>
{
  fn get_messages(server: &S) -> Option<VecDeque<(SocketAddr, I)>> {
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
  fn send_message(server: &S, addr: SocketAddr, msg: O) -> Result<(), InterError> {
    let msg_string = msg.to_raw()?;
    server.send(addr, msg_string + ";")?;
    Ok(())
  }
}
