use thiserror::Error;

pub mod clock;
pub(crate) mod func;

#[derive(Error, Debug)]
pub enum MidiClockError {}

pub type MidiClockResult<A> = Result<A, MidiClockError>;
