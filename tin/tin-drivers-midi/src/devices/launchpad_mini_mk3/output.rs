use crate::devices::launchpad_mini_mk3::visual::LPM3Visual;
use crate::{MidiDriverError, MidiOutputMessage, MidiPhysicalPosition};

#[derive(Debug)]
pub enum LPM3OutputMessage {
    Raw(Vec<u8>),
    SendColors(Vec<LPM3Visual>),
}

impl MidiOutputMessage for LPM3OutputMessage {
    fn to_raw(self) -> Result<Vec<u8>, MidiDriverError> {
        match self {
            Self::Raw(v) => Ok(v),
            Self::SendColors(v) => {
                let mut result = vec![240, 0, 32, 41, 2, 13, 3];
                for visual in v {
                    match visual {
                        LPM3Visual::Off(pos) => result.append(&mut vec![0, pos.to_raw()?, 0]),
                        LPM3Visual::Static(pos, color) => {
                            result.append(&mut vec![0, pos.to_raw()?, color]);
                        }
                        LPM3Visual::Flashing(pos, a, b) => {
                            result.append(&mut vec![1, pos.to_raw()?, a, b]);
                        }
                        LPM3Visual::Pulsing(pos, color) => {
                            result.append(&mut vec![2, pos.to_raw()?, color]);
                        }
                        LPM3Visual::RGB(pos, r, g, b) => {
                            result.append(&mut vec![3, pos.to_raw()?, r, g, b])
                        }
                    }
                }
                result.push(247);
                Ok(result)
            }
        }
    }
}
