use anyhow::Result;
use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect, text::Line};

use crate::EditorData;

pub mod sections;
pub mod select;

pub trait View {
  fn get_title<'a>(&self) -> Line<'a>;
  fn get_instructions<'a>(&self) -> Line<'a>;
  fn handle_key_event(
    &mut self,
    editor: &mut EditorData,
    key_event: KeyEvent,
  ) -> Result<Option<Box<dyn View>>>;
  fn render(&self, editor: &EditorData, area: Rect, buf: &mut Buffer);
}
