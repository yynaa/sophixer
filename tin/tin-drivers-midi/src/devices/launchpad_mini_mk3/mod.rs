use crate::{MidiDriverError, MidiPhysicalPosition};

pub mod driver;
pub mod input;
pub mod output;
pub mod visual;

/// position of a key on the pad
#[derive(Debug, Clone)]
pub enum LPM3Position {
    Raw(u8),
    Grid(u8, u8),
    Up,
    Down,
    Left,
    Right,
    Session,
    Drums,
    Keys,
    User,
    Logo,
    SSM,
}

impl MidiPhysicalPosition for LPM3Position {
    fn to_raw(&self) -> Result<u8, MidiDriverError> {
        match self {
            LPM3Position::Raw(pos) => {
                if *pos % 10 == 0 || *pos > 99 || *pos < 11 {
                    return Err(MidiDriverError::InvalidPosition(
                        "Launchpad Mini MK3".to_string(),
                        pos.to_string(),
                    ));
                }
                Ok(*pos)
            }
            LPM3Position::Grid(x, y) => {
                if *x < 1 || *x > 9 || *y > 8 {
                    return Err(MidiDriverError::InvalidPosition(
                        "Launchpad Mini MK3".to_string(),
                        format!("({}, {})", x, y),
                    ));
                }
                Ok((9 - y) * 10 + x)
            }
            LPM3Position::Up => Ok(91),
            LPM3Position::Down => Ok(92),
            LPM3Position::Left => Ok(93),
            LPM3Position::Right => Ok(94),
            LPM3Position::Session => Ok(95),
            LPM3Position::Drums => Ok(96),
            LPM3Position::Keys => Ok(97),
            LPM3Position::User => Ok(98),
            LPM3Position::Logo => Ok(99),
            LPM3Position::SSM => Ok(19),
        }
    }
}

impl PartialEq for LPM3Position {
    fn eq(&self, other: &Self) -> bool {
        self.to_raw().unwrap() == other.to_raw().unwrap()
    }
}

impl Eq for LPM3Position {}
