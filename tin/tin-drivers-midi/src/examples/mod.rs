//! usage examples
//!
//! to see these in action, encase these functions into loops and pass the appropriate drivers

pub mod lpm3_lcxl2_cube;

pub use lpm3_lcxl2_cube::CubeDemo;

use crate::devices::launch_control_xl_mk2::{LCXL2Driver, LCXL2Position, LCXL2Visual};
use crate::devices::launchpad_mini_mk3::{LPM3Driver, LPM3InputMessage, LPM3Visual};
use crate::{MidiDriver, MidiDriverError, MidiPhysicalState};

/// LCXL2 -- test colors
pub fn lcxl2_color_test(lcxl2: &mut LCXL2Driver) -> Result<(), MidiDriverError> {
    let _ = lcxl2.read()?;
    lcxl2.clear()?;

    {
        let MidiPhysicalState::Analog8(r) = lcxl2.get_position_state(LCXL2Position::Knob(1, 1))?
        else {
            unreachable!()
        };
        let MidiPhysicalState::Analog8(g) = lcxl2.get_position_state(LCXL2Position::Knob(2, 1))?
        else {
            unreachable!()
        };
        lcxl2.add(LCXL2Visual::Static(
            LCXL2Position::Knob(1, 3),
            r / 32,
            g / 32,
        ))?;
    }

    {
        let MidiPhysicalState::Analog8(r) = lcxl2.get_position_state(LCXL2Position::Knob(3, 1))?
        else {
            unreachable!()
        };
        let MidiPhysicalState::Analog8(g) = lcxl2.get_position_state(LCXL2Position::Knob(4, 1))?
        else {
            unreachable!()
        };
        lcxl2.add(LCXL2Visual::Static(
            LCXL2Position::Bottom(1, 1),
            r / 32,
            g / 32,
        ))?;
    }

    {
        let MidiPhysicalState::Analog8(r) = lcxl2.get_position_state(LCXL2Position::Knob(5, 1))?
        else {
            unreachable!()
        };
        let MidiPhysicalState::Analog8(g) = lcxl2.get_position_state(LCXL2Position::Knob(6, 1))?
        else {
            unreachable!()
        };
        lcxl2.add(LCXL2Visual::Static(
            LCXL2Position::Bottom(1, 2),
            r / 32,
            g / 32,
        ))?;
    }

    {
        let MidiPhysicalState::Analog8(r) = lcxl2.get_position_state(LCXL2Position::Knob(1, 2))?
        else {
            unreachable!()
        };
        let MidiPhysicalState::Analog8(g) = lcxl2.get_position_state(LCXL2Position::Knob(2, 2))?
        else {
            unreachable!()
        };
        lcxl2.add(LCXL2Visual::Static(LCXL2Position::Down, r / 32, g / 32))?;
    }

    {
        let MidiPhysicalState::Analog8(r) = lcxl2.get_position_state(LCXL2Position::Knob(3, 2))?
        else {
            unreachable!()
        };
        let MidiPhysicalState::Analog8(g) = lcxl2.get_position_state(LCXL2Position::Knob(4, 2))?
        else {
            unreachable!()
        };
        lcxl2.add(LCXL2Visual::Static(LCXL2Position::Left, r / 32, g / 32))?;
    }

    {
        let MidiPhysicalState::Analog8(r) = lcxl2.get_position_state(LCXL2Position::Knob(5, 2))?
        else {
            unreachable!()
        };
        let MidiPhysicalState::Analog8(g) = lcxl2.get_position_state(LCXL2Position::Knob(6, 2))?
        else {
            unreachable!()
        };
        lcxl2.add(LCXL2Visual::Static(LCXL2Position::Device, r / 32, g / 32))?;
    }

    lcxl2.push()?;

    Ok(())
}

/// LPM3 -- touch test
pub fn lpm3_touch(lpm3: &mut LPM3Driver) -> Result<(), MidiDriverError> {
    let mut d = lpm3.read()?;
    while let Some(msg) = d.pop_front() {
        match msg {
            LPM3InputMessage::KeyPressed(pos) => lpm3.add(LPM3Visual::Flashing(pos, 3, 1))?,
            LPM3InputMessage::KeyReleased(pos) => lpm3.add(LPM3Visual::Off(pos))?,
        }
    }
    lpm3.push()?;
    Ok(())
}
