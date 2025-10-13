use crate::{MidiDriverError, MidiPhysicalPosition};

pub mod driver;
pub mod input;
pub mod output;
pub mod visual;

pub use driver::LCXL2Driver;
pub use input::LCXL2InputMessage;
pub use output::LCXL2OutputMessage;
pub use visual::LCXL2Visual;

/// position of a key on the pad
#[derive(Debug, Clone)]
pub enum LCXL2Position {
    Raw(u8),
    Knob(u8, u8),
    Slider(u8),
    Bottom(u8, u8),
    Up,
    Down,
    Left,
    Right,
    Device,
    Mute,
    Solo,
    Record,
}

impl LCXL2Position {
    #[allow(unused)]
    fn is_analog(&self) -> Result<bool, MidiDriverError> {
        Ok(!(24..48).contains(&self.to_raw()?))
    }

    fn is_analog_raw(raw: &u8) -> bool {
        !(24..48).contains(raw)
    }
}

impl MidiPhysicalPosition for LCXL2Position {
    fn to_raw(&self) -> Result<u8, MidiDriverError> {
        match self {
            LCXL2Position::Raw(pos) => {
                if *pos > 55 {
                    return Err(MidiDriverError::InvalidPosition(
                        "Launch Control XL MK2".to_string(),
                        pos.to_string(),
                    ));
                }
                Ok(*pos)
            }
            LCXL2Position::Knob(x, y) => {
                if *x < 1 || *x > 8 || *y < 1 || *y > 3 {
                    return Err(MidiDriverError::InvalidPosition(
                        "Launch Control XL MK2".to_string(),
                        format!("Knob ({}, {})", x, y),
                    ));
                }
                Ok((y - 1) * 8 + (x - 1))
            }
            LCXL2Position::Slider(pos) => {
                if *pos < 1 || *pos > 8 {
                    return Err(MidiDriverError::InvalidPosition(
                        "Launch Control XL MK2".to_string(),
                        format!("Slider {}", pos),
                    ));
                }
                Ok(47 + *pos)
            }
            LCXL2Position::Bottom(x, y) => {
                if *x < 1 || *x > 8 || *y < 1 || *y > 2 {
                    return Err(MidiDriverError::InvalidPosition(
                        "Launch Control XL MK2".to_string(),
                        format!("Bottom ({}, {})", x, y),
                    ));
                }
                Ok(24 + (y - 1) * 8 + (x - 1))
            }
            LCXL2Position::Up => Ok(44),
            LCXL2Position::Down => Ok(45),
            LCXL2Position::Left => Ok(46),
            LCXL2Position::Right => Ok(47),
            LCXL2Position::Device => Ok(40),
            LCXL2Position::Mute => Ok(41),
            LCXL2Position::Solo => Ok(42),
            LCXL2Position::Record => Ok(43),
        }
    }
}

impl PartialEq for LCXL2Position {
    fn eq(&self, other: &Self) -> bool {
        self.to_raw().unwrap() == other.to_raw().unwrap()
    }
}

impl Eq for LCXL2Position {}
