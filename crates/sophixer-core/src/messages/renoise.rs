use intercom::{InterMessageIncoming, InterMessageOutgoing, InterMessagePrefixed};

#[derive(Debug)]
pub enum MessageFromRenoise {
  Hello,
  Goodbye,
}

impl InterMessagePrefixed for MessageFromRenoise {
  fn get_prefix() -> String {
    String::from("calcium")
  }
}

impl InterMessageIncoming for MessageFromRenoise {
  fn from_raw(raw: Vec<&str>) -> Option<Self> {
    match &raw.len() {
      1 => match raw[0] {
        "hello" => Some(Self::Hello),
        "goodbye" => Some(Self::Goodbye),
        _ => None,
      },
      _ => None,
    }
  }
}

pub enum MessageToRenoise {
  Welcome,
}

impl InterMessageOutgoing for MessageToRenoise {
  fn to_raw(self) -> Result<String, intercom::InterError> {
    match self {
      Self::Welcome => Ok(String::from("welcome")),
    }
  }
}
