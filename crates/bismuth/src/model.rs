use std::collections::HashSet;

use sophixer_core::song_data::Set;

pub struct BismuthModel {
  pub set: Option<Set>,
  pub renoise_instances: HashSet<u64>,
}

impl BismuthModel {
  pub fn new() -> Self {
    Self {
      set: None,
      renoise_instances: HashSet::new(),
    }
  }
}
