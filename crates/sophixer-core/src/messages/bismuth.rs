use intercom::{InterMessageIncoming, InterMessageOutgoing, InterMessagePrefixed};

use crate::song_data::{Set, Song};

#[derive(Debug)]
pub enum MessageFromBismuth {
  Hello,
  Goodbye,
}

impl InterMessagePrefixed for MessageFromBismuth {
  fn get_prefix() -> String {
    String::from("bismuth")
  }
}

impl InterMessageIncoming for MessageFromBismuth {
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

impl InterMessageOutgoing for MessageFromBismuth {
  fn to_raw(self) -> Result<String, intercom::InterError> {
    match self {
      Self::Hello => Ok(String::from("hello")),
      Self::Goodbye => Ok(String::from("goodbye")),
    }
  }
}

pub enum MessageToBismuth {
  Welcome,
  InitSet {
    name: String,
    authors: String,
  },
  InitSong {
    id: String,
    name: String,
    authors: String,
    order: i64,
    color: (u8, u8, u8),
  },
  RenoiseInstanceAdded(u64),
  RenoiseInstanceRemoved(u64),
}

impl InterMessageIncoming for MessageToBismuth {
  fn from_raw(raw: Vec<&str>) -> Option<Self> {
    match raw.len() {
      1 => match raw.get(0).unwrap() {
        &"welcome" => Some(Self::Welcome),
        _ => None,
      },
      2 => match raw[0] {
        "renoiseInstanceAdded" => {
          if let Ok(id) = raw[1].parse::<u64>() {
            Some(Self::RenoiseInstanceAdded(id))
          } else {
            None
          }
        }
        "renoiseInstanceRemoved" => {
          if let Ok(id) = raw[1].parse::<u64>() {
            Some(Self::RenoiseInstanceRemoved(id))
          } else {
            None
          }
        }
        _ => None,
      },
      3 => match raw[0] {
        "initSet" => Some(Self::InitSet {
          name: raw[1].to_string(),
          authors: raw[2].to_string(),
        }),
        _ => None,
      },
      8 => match raw[0] {
        "initSong" => {
          let order_option = raw[4].parse::<i64>();
          let color_red_option = raw[5].parse::<u8>();
          let color_green_option = raw[6].parse::<u8>();
          let color_blue_option = raw[7].parse::<u8>();

          let option = order_option.and_then(|order| {
            color_red_option.and_then(|color_red| {
              color_green_option.and_then(|color_green| {
                color_blue_option
                  .and_then(|color_blue| Ok((order, color_red, color_green, color_blue)))
              })
            })
          });

          if let Ok((order, color_red, color_green, color_blue)) = option {
            Some(Self::InitSong {
              id: raw[1].to_string(),
              name: raw[2].to_string(),
              authors: raw[3].to_string(),
              order,
              color: (color_red, color_green, color_blue),
            })
          } else {
            None
          }
        }
        _ => None,
      },
      _ => None,
    }
  }
}

impl InterMessageOutgoing for MessageToBismuth {
  fn to_raw(self) -> Result<String, intercom::InterError> {
    match self {
      Self::Welcome => Ok(String::from("welcome")),
      Self::InitSet { name, authors } => Ok(format!("initSet:{}:{}", name, authors)),
      Self::InitSong {
        id,
        name,
        authors,
        order,
        color,
      } => Ok(format!(
        "initSong:{}:{}:{}:{}:{}:{}:{}",
        id, name, authors, order, color.0, color.1, color.2
      )),
      Self::RenoiseInstanceAdded(socket) => Ok(format!("renoiseInstanceAdded:{}", socket)),
      Self::RenoiseInstanceRemoved(socket) => Ok(format!("renoiseInstanceRemoved:{}", socket)),
    }
  }
}

impl From<Set> for MessageToBismuth {
  fn from(value: Set) -> Self {
    Self::InitSet {
      name: value.name,
      authors: value.authors,
    }
  }
}

impl TryFrom<MessageToBismuth> for Set {
  type Error = anyhow::Error;

  fn try_from(value: MessageToBismuth) -> Result<Self, Self::Error> {
    match value {
      MessageToBismuth::InitSet { name, authors } => Set::new(name, authors),
      _ => Err(anyhow::Error::msg("invalid message, not a set")),
    }
  }
}

impl TryFrom<MessageToBismuth> for Song {
  type Error = anyhow::Error;

  fn try_from(value: MessageToBismuth) -> Result<Self, Self::Error> {
    match value {
      MessageToBismuth::InitSong {
        id: _id,
        name,
        authors,
        order,
        color,
      } => {
        let mut song = Song::new(name, authors, String::new())?;
        song.order = order;
        song.color = color;
        Ok(song)
      }
      _ => Err(anyhow::Error::msg("invalid message, not a set")),
    }
  }
}
