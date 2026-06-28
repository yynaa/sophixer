use anyhow::Result;

use crate::messages::renoise;

impl renoise::from::MessageFromRenoise {
  pub fn build(t: impl Into<renoise::from::message_from_renoise::FromMessage>) -> Result<Self> {
    Ok(Self {
      from_message: Some(t.into()),
    })
  }

  pub fn unpack(&self) -> Result<renoise::from::message_from_renoise::FromMessage> {
    Ok(
      self
        .from_message
        .ok_or(anyhow::Error::msg("couldn't get inner message"))?,
    )
  }
}

impl renoise::to::MessageToRenoise {
  pub fn build(t: impl Into<renoise::to::message_to_renoise::ToMessage>) -> Result<Self> {
    Ok(Self {
      to_message: Some(t.into()),
    })
  }

  pub fn unpack(&self) -> Result<renoise::to::message_to_renoise::ToMessage> {
    Ok(
      self
        .to_message
        .clone()
        .ok_or(anyhow::Error::msg("couldn't get inner message"))?,
    )
  }
}
