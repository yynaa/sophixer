use crate::devices::launch_control_xl_mk2::LCXL2Position;
use crate::MidiInputMessage;

#[derive(Debug)]
pub enum LCXL2InputMessage {
    KeyPressed(LCXL2Position),
    KeyReleased(LCXL2Position),
    Analog(LCXL2Position, u8),
}

impl MidiInputMessage for LCXL2InputMessage {
    fn from_raw(raw: &[u8]) -> Option<Self> {
        if raw.len() == 3 {
            let ty = raw[0];
            if ty == 176 || ty == 144 {
                let pos = raw[1];
                let value = raw[2];
                if LCXL2Position::is_analog_raw(&pos) {
                    Some(LCXL2InputMessage::Analog(LCXL2Position::Raw(pos), value))
                } else {
                    if value > 0 {
                        Some(LCXL2InputMessage::KeyPressed(LCXL2Position::Raw(pos)))
                    } else {
                        Some(LCXL2InputMessage::KeyReleased(LCXL2Position::Raw(pos)))
                    }
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}
