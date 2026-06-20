use std::{
  sync::mpsc,
  thread::{self, JoinHandle},
};

use crate::{func::thread_function, MidiClockResult};

pub enum VirtualMidiClockMessage {
  Stop,
}

pub struct VirtualMidiClock {
  tx: mpsc::Sender<VirtualMidiClockMessage>,
  handle: JoinHandle<()>,
}

impl VirtualMidiClock {
  pub fn new(id: u64) -> MidiClockResult<Self> {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || thread_function());
    Ok(Self { tx, handle })
  }
}
