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
  LoadSong(String),
  PlaySection(u64, u64),
  StopTransport,
  MuteTrack(u64, bool),
  MuteTrackSequenceSlot(u64, u64, bool),
  BypassEffect(u64, u64, bool),
  SetParameterValue(u64, u64, u64, f64),
}

impl InterMessageOutgoing for MessageToRenoise {
  fn to_raw(self) -> Result<String, intercom::InterError> {
    match self {
      Self::Welcome => Ok(String::from("welcome")),
      Self::LoadSong(s) => Ok(format!("loadSong,{}", s)),
      Self::PlaySection(s, l) => Ok(format!("playSection,{},{}", s + 1, l)),
      Self::StopTransport => Ok(String::from("stopTransport")),
      Self::MuteTrack(t, b) => Ok(format!("muteTrack,{},{}", t, if b { 0 } else { 1 })),
      Self::MuteTrackSequenceSlot(t, s, b) => Ok(format!(
        "muteTrackSequenceSlot,{},{},{}",
        t,
        s + 1,
        if b { 0 } else { 1 }
      )),
      Self::BypassEffect(t, e, b) => Ok(format!(
        "bypassEffect,{},{},{}",
        t,
        e,
        if b { 0 } else { 1 }
      )),
      Self::SetParameterValue(t, e, p, v) => {
        Ok(format!("setParameterValue,{},{},{},{}", t, e, p, v))
      }
    }
  }
}
