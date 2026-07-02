use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Ord)]
pub enum Channel {
  Lead(u16),
  Drum(u16),
  MasterLead,
  MasterDrum,
  Master,
}

impl Default for Channel {
  fn default() -> Self {
    Self::Master
  }
}

impl Channel {
  pub fn to_renoise_number(&self) -> u16 {
    match self {
      Self::Lead(n) => *n,
      Self::Drum(n) => 7 + *n,
      Self::MasterLead => 7,
      Self::MasterDrum => 14,
      Self::Master => 15,
    }
  }
}

impl Display for Channel {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Lead(n) => write!(f, "Lead{}", n),
      Self::Drum(n) => write!(f, "Drum{}", n),
      Self::MasterLead => write!(f, "MasterLead"),
      Self::MasterDrum => write!(f, "MasterDrum"),
      Self::Master => write!(f, "Master"),
    }
  }
}

impl PartialOrd for Channel {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.to_renoise_number().cmp(&other.to_renoise_number()))
  }
}
