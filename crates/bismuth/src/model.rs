use std::collections::HashSet;

use sophixer_core::song_data::Set;

pub struct BismuthModel {
  pub set: Option<Set>,
  pub renoise_instances: HashSet<u64>,

  pub renoise_instance_selector: Option<u64>,
  pub song_selector: Option<String>,
}

impl BismuthModel {
  pub fn new() -> Self {
    Self {
      set: None,
      renoise_instances: HashSet::new(),
      renoise_instance_selector: None,
      song_selector: None,
    }
  }
}
