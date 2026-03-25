pub mod udp;

use log::warn;
use std::collections::VecDeque;

use crate::{InterError, InterMessageIncoming, InterMessageOutgoing, InterMessagePrefixed};

pub trait InterClient: Sized {
  fn start(addr: &str) -> Result<Self, InterError>;
  fn stop(self) -> Result<(), InterError>;
  fn send(&self, msg: String) -> Result<(), InterError>;
  fn fetch(&mut self) -> Result<(), InterError>;
  fn get(&self) -> Option<&VecDeque<String>>;
}

pub trait InterClientCommunicator<
  C: InterClient,
  I: InterMessageIncoming,
  O: InterMessageOutgoing + InterMessagePrefixed,
>
{
  fn get_messages(client: &C) -> Option<VecDeque<I>> {
    client.get().map(|deque| {
      let mut deque_clone = deque.clone();
      let mut r = VecDeque::new();
      while let Some(msg_string) = deque_clone.pop_front() {
        match I::from_raw(msg_string.split(":").collect()) {
          None => {
            warn!("unrecognized message from server: {msg_string:?}")
          }
          Some(msg) => {
            r.push_back(msg);
          }
        }
      }
      r
    })
  }
  fn send_message(client: &C, msg: O) -> Result<(), InterError> {
    let msg_string = msg.to_raw()?;
    client.send(O::get_prefix() + ":" + &msg_string + ";")?;
    Ok(())
  }
}
