use crate::MidiDriverError;
use midir::{Ignore, MidiInput, MidiInputPort, MidiOutput, MidiOutputPort};

pub mod launchpad_mini_mk3;

pub fn get_in_port(device: &str) -> Result<(MidiInput, MidiInputPort), MidiDriverError> {
    let mut midi_in = MidiInput::new(&format!("{} tin-driver input", device))
        .map_err(MidiDriverError::MidirInitError)?;
    midi_in.ignore(Ignore::None);
    let in_ports = midi_in.ports();
    if in_ports.is_empty() {
        return Err(MidiDriverError::MidiNotFound(device.to_string()));
    }
    for p in in_ports {
        let port_name = midi_in
            .port_name(&p)
            .map_err(MidiDriverError::MidirPortInfoError)?;
        if port_name.contains(device) {
            return Ok((midi_in, p));
        }
    }
    Err(MidiDriverError::MidiNotFound(device.to_string()))
}

pub fn get_out_port(device: &str) -> Result<(MidiOutput, MidiOutputPort), MidiDriverError> {
    let midi_out = MidiOutput::new(&format!("{} tin-driver output", device))
        .map_err(MidiDriverError::MidirInitError)?;
    let out_ports = midi_out.ports();
    if out_ports.is_empty() {
        return Err(MidiDriverError::MidiNotFound(device.to_string()));
    }
    for p in out_ports {
        let port_name = midi_out
            .port_name(&p)
            .map_err(MidiDriverError::MidirPortInfoError)?;
        if port_name.contains(device) {
            return Ok((midi_out, p));
        }
    }
    Err(MidiDriverError::MidiNotFound(device.to_string()))
}
