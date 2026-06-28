use intercom::{InterMessageIncoming, InterMessageOutgoing, InterMessagePrefixed};

pub mod helpers;

pub mod renoise {
  pub mod from {
    include!(concat!(env!("OUT_DIR"), "/renoise.from.rs"));
  }
  pub mod to {
    include!(concat!(env!("OUT_DIR"), "/renoise.to.rs"));
  }
}

use prost::Message;
use renoise::from::MessageFromRenoise;
use renoise::to::MessageToRenoise;

impl InterMessagePrefixed for MessageFromRenoise {
  fn get_prefix() -> u8 {
    2
  }
}

impl InterMessageIncoming for MessageFromRenoise {
  fn deserialize(bytes: Vec<u8>) -> Option<Self> {
    MessageFromRenoise::decode(bytes.as_slice()).ok()
  }
}

impl InterMessageOutgoing for MessageToRenoise {
  fn serialize(&self) -> Option<Vec<u8>> {
    let mut buf = Vec::new();
    buf.reserve(self.encoded_len());
    match self.encode(&mut buf) {
      Ok(()) => Some(buf),
      Err(_) => None,
    }
  }
}
