pub mod udp;

use std::{collections::VecDeque, marker::PhantomData, net::SocketAddr};

use crate::{InterError, InterMessageIncoming, InterMessageOutgoing, InterMessagePrefixed};

#[async_trait::async_trait]
pub trait InterServer: Sized {
  async fn start(addr: &str) -> Result<Self, InterError>;
  async fn stop(self) -> Result<(), InterError>;
  async fn send(&self, addr: SocketAddr, msg: &[u8]) -> Result<(), InterError>;
  async fn fetch(&mut self) -> Result<(), InterError>;
  fn get(&self, prefix: u8) -> Option<&VecDeque<(SocketAddr, Vec<u8>)>>;
}

pub struct InterServerCommunicator<S, I, O>
where
  S: InterServer,
  I: InterMessageIncoming + InterMessagePrefixed,
  O: InterMessageOutgoing,
{
  s: PhantomData<S>,
  i: PhantomData<I>,
  o: PhantomData<O>,
}

impl<'de, S: InterServer, I: InterMessageIncoming + InterMessagePrefixed, O: InterMessageOutgoing>
  InterServerCommunicator<S, I, O>
{
  pub fn get_messages(server: &S) -> Option<VecDeque<(SocketAddr, I)>> {
    server.get(I::get_prefix()).map(|deque| {
      let mut deque_clone = deque.clone();
      let mut r = VecDeque::new();
      while let Some((addr, msg)) = deque_clone.pop_front() {
        // match  {
        //   None => {
        //     warn!("unrecognized message from server: {msg_string:?}")
        //   }
        //   Some(msg) => {
        //     r.push_back((addr, msg));
        //   }
        // }
        if let Some(msg) = I::deserialize(msg.clone()) {
          trace!("received: {:?}", msg);
          r.push_back((addr, msg));
        } else {
          warn!("unrecognized message from client {}: {:?}", addr, msg);
        }
      }
      r
    })
  }
  pub async fn send_message(server: &S, addr: SocketAddr, msg: O) -> Result<(), InterError> {
    trace!("sending: {:?}", msg);
    let msg_string = msg.serialize().ok_or(InterError::NoSerialization)?;
    server.send(addr, &msg_string).await?;
    Ok(())
  }
}
