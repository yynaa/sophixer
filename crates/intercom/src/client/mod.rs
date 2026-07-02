pub mod udp;

use log::warn;
use std::{collections::VecDeque, marker::PhantomData};

use crate::{InterError, InterMessageIncoming, InterMessageOutgoing, InterMessagePrefixed};

#[async_trait::async_trait]
pub trait InterClient: Sized {
  async fn start(addr: &str) -> Result<Self, InterError>;
  async fn stop(self) -> Result<(), InterError>;
  async fn send(&self, msg: &[u8]) -> Result<(), InterError>;
  async fn fetch(&mut self) -> Result<(), InterError>;
  fn get(&self) -> Option<&VecDeque<Vec<u8>>>;
}

pub struct InterClientCommunicator<C, I, O>
where
  C: InterClient,
  I: InterMessageIncoming,
  O: InterMessageOutgoing + InterMessagePrefixed,
{
  c: PhantomData<C>,
  i: PhantomData<I>,
  o: PhantomData<O>,
}

impl<'de, C: InterClient, I: InterMessageIncoming, O: InterMessageOutgoing + InterMessagePrefixed>
  InterClientCommunicator<C, I, O>
{
  pub fn get_messages(client: &C) -> Option<VecDeque<I>> {
    client.get().map(|deque| {
      let mut deque_clone = deque.clone();
      let mut r = VecDeque::new();
      while let Some(msg) = deque_clone.pop_front() {
        if let Some(msg) = I::deserialize(msg.clone()) {
          trace!("received: {:?}", msg);
          r.push_back(msg);
        } else {
          warn!("unrecognized message from server: {:?}", msg);
        }
      }
      r
    })
  }
  pub async fn send_message(client: &C, msg: O) -> Result<(), InterError> {
    let mut bytes = msg.serialize().ok_or(InterError::NoSerialization)?.to_vec();
    bytes.insert(0, O::get_prefix());
    client.send(&bytes).await?;
    Ok(())
  }
}
