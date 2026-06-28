pub mod udp;

use serde_value::Value;
use std::{collections::VecDeque, marker::PhantomData, net::SocketAddr};

use crate::{InterError, InterMessageIncoming, InterMessageOutgoing, InterMessagePrefixed};

#[async_trait::async_trait]
pub trait InterServer: Sized {
  async fn start(addr: &str) -> Result<Self, InterError>;
  async fn stop(self) -> Result<(), InterError>;
  async fn send(&self, addr: SocketAddr, msg: String) -> Result<(), InterError>;
  async fn fetch(&mut self) -> Result<(), InterError>;
  fn get(&self, prefix: String) -> Option<&VecDeque<(SocketAddr, Value)>>;
}

pub struct InterServerCommunicator<'de, S, I, O>
where
  S: InterServer,
  I: InterMessageIncoming<'de> + InterMessagePrefixed,
  O: InterMessageOutgoing,
{
  s: PhantomData<S>,
  i: PhantomData<&'de I>,
  o: PhantomData<O>,
}

impl<
  'de,
  S: InterServer,
  I: InterMessageIncoming<'de> + InterMessagePrefixed,
  O: InterMessageOutgoing,
> InterServerCommunicator<'de, S, I, O>
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
        if let Ok(msg) = msg.clone().deserialize_into::<I>() {
          r.push_back((addr, msg));
        } else {
          warn!("unrecognized message from client {}: {:?}", addr, msg);
        }
      }
      r
    })
  }
  pub async fn send_message(server: &S, addr: SocketAddr, msg: O) -> Result<(), InterError> {
    let msg_string = serde_json::to_string(&msg)?;
    server.send(addr, msg_string).await?;
    Ok(())
  }
}
