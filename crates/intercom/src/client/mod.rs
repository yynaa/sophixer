pub mod udp;

use log::warn;
use serde_value::{Value, to_value};
use std::{collections::VecDeque, marker::PhantomData};

use crate::{
  InterError, InterMessageIncoming, InterMessageOutgoing, InterMessagePrefixed, PrefixedMessage,
};

#[async_trait::async_trait]
pub trait InterClient: Sized {
  async fn start(addr: &str) -> Result<Self, InterError>;
  async fn stop(self) -> Result<(), InterError>;
  async fn send(&self, msg: String) -> Result<(), InterError>;
  async fn fetch(&mut self) -> Result<(), InterError>;
  fn get(&self) -> Option<&VecDeque<Value>>;
}

pub struct InterClientCommunicator<'de, C, I, O>
where
  C: InterClient,
  I: InterMessageIncoming<'de>,
  O: InterMessageOutgoing + InterMessagePrefixed,
{
  c: PhantomData<C>,
  i: PhantomData<&'de I>,
  o: PhantomData<O>,
}

impl<
  'de,
  C: InterClient,
  I: InterMessageIncoming<'de>,
  O: InterMessageOutgoing + InterMessagePrefixed,
> InterClientCommunicator<'de, C, I, O>
{
  pub fn get_messages(client: &C) -> Option<VecDeque<I>> {
    client.get().map(|deque| {
      let mut deque_clone = deque.clone();
      let mut r = VecDeque::new();
      while let Some(msg) = deque_clone.pop_front() {
        if let Ok(msg) = msg.clone().deserialize_into::<I>() {
          r.push_back(msg);
        } else {
          warn!("unrecognized message from server: {:?}", msg);
        }
      }
      r
    })
  }
  pub async fn send_message(client: &C, msg: O) -> Result<(), InterError> {
    let prefixed = PrefixedMessage {
      prefix: O::get_prefix(),
      message: to_value(&msg)?,
    };
    let msg_string = serde_json::to_string(&prefixed)?;
    client.send(msg_string).await?;
    Ok(())
  }
}
