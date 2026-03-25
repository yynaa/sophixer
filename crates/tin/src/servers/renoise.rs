use anyhow::Result;

use intercom::{InterCommunicator, InterMessageIncoming, InterMessageOutgoing, Server};

use crate::model::{RenoiseInstance, TinModel};

#[derive(Debug)]
pub enum RenoiseMessageIncoming {
  Hello,
  Goodbye,
}

impl InterMessageIncoming for RenoiseMessageIncoming {
  fn get_prefix() -> String {
    String::from("renoise")
  }

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

pub enum RenoiseMessageOutgoing {
  Welcome,
}

impl InterMessageOutgoing for RenoiseMessageOutgoing {
  fn to_raw(self) -> Result<String, intercom::InterError> {
    match self {
      Self::Welcome => Ok(String::from("welcome")),
    }
  }
}

pub struct RenoiseCommunicator {}
impl InterCommunicator<RenoiseMessageIncoming, RenoiseMessageOutgoing> for RenoiseCommunicator {}

pub fn update_model_from_renoise(model: &mut TinModel, server: &Server) -> Result<()> {
  let messages = RenoiseCommunicator::get_messages(server);
  if let Some(messages) = messages {
    for (from, msg) in messages {
      match msg {
        RenoiseMessageIncoming::Hello => {
          model.instances.insert(from, RenoiseInstance::new());
          RenoiseCommunicator::send_message(server, from, RenoiseMessageOutgoing::Welcome)?;
          info!("new renoise instance from {}", from);
        }
        RenoiseMessageIncoming::Goodbye => {
          model.instances.remove(&from);
          info!("renoise instance from {} disconnected", from);
        }
      }
    }
  }

  Ok(())
}
