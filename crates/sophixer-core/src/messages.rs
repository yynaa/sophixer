use intercom::{InterMessageIncoming, InterMessageOutgoing, InterMessagePrefixed};

pub mod renoise {
  pub mod from {
    include!(concat!(env!("OUT_DIR"), "/message_from_renoise.rs"));
  }
  pub mod to {
    include!(concat!(env!("OUT_DIR"), "/message_to_renoise.rs"));
  }
}

use renoise::from::MessageFromRenoise;
use renoise::to::MessageToRenoise;

impl InterMessagePrefixed for MessageFromRenoise {
  fn get_prefix() -> u8 {
    2
  }
}

impl InterMessageIncoming for MessageFromRenoise {
  fn deserialize(bytes: Vec<u8>) -> Option<Self> {
    MessageFromRenoise::deserialize(bytes)
  }
}

impl InterMessageOutgoing for MessageToRenoise {
  fn serialize(&self) -> Option<Vec<u8>> {
    Some(self.serialize())
  }
}
