use crate::devices::launchpad_mini_mk3::LPM3Position;
use crate::{MidiDriverError, MidiPhysicalPosition, MidiVisual};

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum LPM3Visual {
    Off(LPM3Position),
    Static(LPM3Position, u8),
    Flashing(LPM3Position, u8, u8),
    Pulsing(LPM3Position, u8),
    RGB(LPM3Position, u8, u8, u8),
}

impl MidiVisual for LPM3Visual {
    fn get_raw_pos(&self) -> Result<u8, MidiDriverError> {
        match self {
            Self::Off(p) => p.to_raw(),
            Self::Static(p, _) => p.to_raw(),
            Self::Flashing(p, _, _) => p.to_raw(),
            Self::Pulsing(p, _) => p.to_raw(),
            Self::RGB(p, _, _, _) => p.to_raw(),
        }
    }

    fn is_default(&self) -> bool {
        match self {
            Self::Off(_) => true,
            Self::Static(_, v) => *v == 0,
            Self::Flashing(_, a, b) => *a == 0 && *b == 0,
            Self::Pulsing(_, v) => *v == 0,
            Self::RGB(_, r, g, b) => *r == 0 && *g == 0 && *b == 0,
        }
    }

    fn is_absorbed(&self, absorber: &Self) -> bool {
        self == absorber
    }
}
