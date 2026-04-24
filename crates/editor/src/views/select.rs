use anyhow::Result;
use crossterm::event::KeyCode;
use ratatui::{
  buffer::Buffer,
  layout::Rect,
  style::Stylize,
  text::{Line, Text},
  widgets::Widget,
};
use sophixer_core::song_data::Song;

use crate::{
  views::{sections::ViewSections, View},
  EditorData,
};

pub struct ViewSelect {
  state: usize,
}

impl ViewSelect {
  pub fn new() -> Result<Self> {
    let s = Self { state: 0 };

    Ok(s)
  }
}

impl View for ViewSelect {
  fn get_title<'a>(&self) -> Line<'a> {
    Line::from(" Song Selector ".bold())
  }

  fn get_instructions<'a>(&self) -> Line<'a> {
    Line::from(vec![" Edit Song ".into(), "<E> ".blue().bold()])
  }

  fn handle_key_event(
    &mut self,
    editor: &mut EditorData,
    key_event: crossterm::event::KeyEvent,
  ) -> Result<Option<Box<dyn View>>> {
    match key_event.code {
      KeyCode::Up => {
        if self.state > 0 {
          self.state -= 1;
        }
        Ok(None)
      }
      KeyCode::Down => {
        if self.state < editor.set.songs.len() - 1 {
          self.state += 1;
        }
        Ok(None)
      }
      KeyCode::Char('e') => {
        let mut sorted = editor.set.songs.iter().collect::<Vec<(&String, &Song)>>();
        sorted.sort_by_key(|f| f.1.order);
        Ok(Some(Box::new(ViewSections::new(
          sorted[self.state].0.clone(),
          (1, 1),
        )?)))
      }
      _ => Ok(None),
    }
  }

  fn render(&self, editor: &EditorData, area: Rect, buf: &mut Buffer) {
    let mut sorted = editor.set.songs.iter().collect::<Vec<(&String, &Song)>>();
    sorted.sort_by_key(|f| f.1.order);
    let items = sorted
      .iter()
      .enumerate()
      .map(|f| {
        let mut sep = "  ";
        if f.0 == self.state {
          sep = ">>";
        }
        format!("{} {}", sep, f.1 .0).into()
      })
      .collect::<Vec<Line>>();

    let text = Text::from(items);
    text.render(area, buf);
  }
}
