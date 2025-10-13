//! *this crate contains drivers for accessing midi devices*
//!
//! you can find examples [here](crate::examples)

#[macro_use]
extern crate log;
use std::collections::VecDeque;
use std::fmt::Debug;
use thiserror::Error;

pub mod devices;
#[allow(unused)]
#[cfg(feature = "examples")]
pub mod examples;

/// trait for device position transformers
pub trait MidiPhysicalPosition: Debug {
    fn to_raw(&self) -> Result<u8, MidiDriverError>;
}

/// enum for a physical position's state
#[derive(Clone, Debug)]
pub enum MidiPhysicalState {
    Binary(bool),
    Analog8(u8),
}

/// trait for wrapping midi messages from an input device
pub trait MidiInputMessage: Sized {
    fn from_raw(raw: &[u8]) -> Option<Self>;
}

/// trait for wrapping midi messages from an input device
pub trait MidiOutputMessage: Sized {
    fn to_raw(self) -> Result<Vec<u8>, MidiDriverError>;
}

/// trait for light & visual elements on the device itself
pub trait MidiVisual: Sized {
    fn get_raw_pos(&self) -> Result<u8, MidiDriverError>;

    fn is_default(&self) -> bool;

    fn is_absorbed(&self, absorber: &Self) -> bool;
}

/// trait for drivers
///
/// visual transformations are made like this, because a minimal amount of midi messages must be sent to avoid blinking
/// this greatly simplifies code down the line!
pub trait MidiDriver<
    I: MidiInputMessage,
    O: MidiOutputMessage,
    V: MidiVisual,
    P: MidiPhysicalPosition,
>: Sized
{
    /// connect to the device
    fn connect() -> Result<Self, MidiDriverError>;
    /// close contact with the device (ensures the device does not get stuck)
    fn close(&mut self) -> Result<(), MidiDriverError>;

    /// read all input messages
    fn read(&mut self) -> Result<VecDeque<I>, MidiDriverError>;
    /// get position state
    fn get_position_state(&self, pos: P) -> Result<MidiPhysicalState, MidiDriverError>;

    /// send an output message
    fn send(&mut self, msg: O) -> Result<(), MidiDriverError>;

    /// pops all visual transformations from the driver
    fn pop(&mut self);
    /// pushes all visual transformations to the device
    fn push(&mut self) -> Result<(), MidiDriverError>;

    /// clear the visuals to default
    fn clear(&mut self) -> Result<(), MidiDriverError>;
    /// add a visual transformation and absorb it with previous state
    fn add(&mut self, visual: V) -> Result<(), MidiDriverError>;
}

/// error type
#[derive(Error, Debug)]
pub enum MidiDriverError {
    /// midir init error
    #[error("midir init error: {0}")]
    MidirInitError(midir::InitError),

    /// midir port info error
    #[error("midir port info error: {0}")]
    MidirPortInfoError(midir::PortInfoError),

    /// midir connection input error
    #[error("midir connection input error: {0}")]
    MidirConnectInputError(midir::ConnectError<midir::MidiInput>),

    /// midir connection output error
    #[error("midir connection output error: {0}")]
    MidirConnectOutputError(midir::ConnectError<midir::MidiOutput>),

    /// midir send error
    #[error("midir send error: {0}")]
    MidirSendError(midir::SendError),

    /// midi device not found
    #[error("midi device not found: {0}")]
    MidiNotFound(String),

    /// invalid position in position transformer
    #[error("invalid position on device {0}: {1}")]
    InvalidPosition(String, String),

    /// invalid visual
    #[error("invalid visual on device {0}: {1}")]
    InvalidVisual(String, String),
}
