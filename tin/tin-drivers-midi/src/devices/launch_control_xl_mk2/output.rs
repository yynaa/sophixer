use crate::devices::launch_control_xl_mk2::visual::LCXL2Visual;
use crate::{MidiDriverError, MidiOutputMessage, MidiPhysicalPosition};

#[derive(Debug)]
pub enum LCXL2OutputMessage {
    Raw(Vec<u8>),
    SendColors(Vec<LCXL2Visual>),
}

impl MidiOutputMessage for LCXL2OutputMessage {
    fn to_raw(self) -> Result<Vec<u8>, MidiDriverError> {
        match self {
            Self::Raw(v) => Ok(v),
            Self::SendColors(v) => {
                // template is not taken into account with this
                let mut result = vec![240, 0, 32, 41, 2, 17, 120, 0];
                for visual in v {
                    match visual {
                        LCXL2Visual::Off(pos) => result.append(&mut vec![pos.to_raw()?, 12]),
                        LCXL2Visual::Static(pos, r, g) => {
                            if r > 3 {
                                return Err(MidiDriverError::InvalidVisual(
                                    "Launch Control XL MK2".to_string(),
                                    format!("Invalid red value: {}", r),
                                ));
                            }
                            if g > 3 {
                                return Err(MidiDriverError::InvalidVisual(
                                    "Launch Control XL MK2".to_string(),
                                    format!("Invalid green value: {}", g),
                                ));
                            }
                            let color = 12 + r + 16 * g;
                            result.append(&mut vec![pos.to_raw()?, color]);
                        }
                    }
                }
                result.push(247);
                Ok(result)
            }
        }
    }
}
