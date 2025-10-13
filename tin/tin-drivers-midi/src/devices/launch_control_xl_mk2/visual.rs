use crate::devices::launch_control_xl_mk2::LCXL2Position;
use crate::{MidiDriverError, MidiPhysicalPosition, MidiVisual};

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum LCXL2Visual {
    Off(LCXL2Position),
    Static(LCXL2Position, u8, u8),
}

impl MidiVisual for LCXL2Visual {
    fn get_raw_pos(&self) -> Result<u8, MidiDriverError> {
        match self {
            Self::Off(p) => p.to_raw(),
            Self::Static(p, _, _) => p.to_raw(),
        }
    }

    fn is_default(&self) -> bool {
        match self {
            Self::Off(_) => true,
            Self::Static(_, r, g) => *r == 0 && *g == 0,
        }
    }

    fn is_absorbed(&self, absorber: &Self) -> bool {
        self == absorber
    }
}
