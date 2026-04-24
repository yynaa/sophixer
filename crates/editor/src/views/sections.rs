use anyhow::Result;
use crossterm::event::KeyCode;
use ratatui::{
  buffer::Buffer,
  layout::Rect,
  style::{Color, Stylize},
  text::Line,
  widgets::{
    canvas::{Canvas, Circle, Rectangle},
    Widget,
  },
};
use sophixer_core::song_data::SongButtonAction;

use crate::{
  views::{select::ViewSelect, View},
  EditorData,
};

pub struct ViewSections {
  song: String,
  cam: (i64, i64),
  hovered: (i64, i64),
}

impl ViewSections {
  pub fn new(song: String, hov: (i64, i64)) -> Result<Self> {
    let s = Self {
      song,
      cam: hov,
      hovered: hov,
    };

    Ok(s)
  }
}

impl View for ViewSections {
  fn get_title<'a>(&self) -> Line<'a> {
    Line::from(" Section Editor ".bold())
  }

  fn get_instructions<'a>(&self) -> Line<'a> {
    Line::from(vec![
      " Back ".into(),
      "<Backspace> ".blue().bold(),
      " New ToggleChannels ".into(),
      "<R> ".blue().bold(),
    ])
  }

  fn handle_key_event(
    &mut self,
    editor: &mut EditorData,
    key_event: crossterm::event::KeyEvent,
  ) -> Result<Option<Box<dyn View>>> {
    match key_event.code {
      KeyCode::Backspace => Ok(Some(Box::new(ViewSelect::new()?))),
      KeyCode::Left => {
        self.hovered.0 -= 1;
        if self.cam.0 > self.hovered.0 {
          self.cam.0 -= 1;
        }
        Ok(None)
      }
      KeyCode::Right => {
        self.hovered.0 += 1;
        if self.cam.0 < self.hovered.0 - 7 {
          self.cam.0 += 1;
        }
        Ok(None)
      }
      KeyCode::Up => {
        self.hovered.1 -= 1;
        if self.cam.1 > self.hovered.1 {
          self.cam.1 -= 1;
        }
        Ok(None)
      }
      KeyCode::Down => {
        self.hovered.1 += 1;
        if self.cam.1 < self.hovered.1 - 7 {
          self.cam.1 += 1;
        }
        Ok(None)
      }
      _ => Ok(None),
    }
  }

  fn render(&self, editor: &EditorData, area: Rect, buf: &mut Buffer) {
    let canvas = Canvas::default()
      .x_bounds([-1., 9.])
      .y_bounds([-8., 2.])
      .paint(|ctx| {
        let song = editor
          .set
          .songs
          .get(&self.song)
          .ok_or(anyhow::Error::msg("couldn't find song"))
          .unwrap();

        for bx in 0..8 {
          ctx.print(bx as f64 + 0.5, 1.1, format!("{}", bx + self.cam.0));
        }
        for y in self.cam.1..self.cam.1 + 8 {
          let by = y - self.cam.1;
          for bx in 0..8 {
            ctx.draw(&Rectangle {
              x: bx as f64,
              y: -by as f64,
              width: 1.,
              height: 1.,
              color: Color::Blue,
            });
            ctx.print(-0.1, -by as f64 + 0.5, format!("{}", y));
          }
          if let Some(section) = song.sections.get(&y) {
            for bx in 0..8 {
              let x = bx + self.cam.0;
              if let Some(button) = section.buttons.get(&x) {
                match button.action {
                  SongButtonAction::ToggleChannels {
                    channels: _,
                    default,
                    color_off,
                    color_on,
                  } => {
                    let color = match default {
                      true => color_on,
                      false => color_off,
                    };
                    ctx.draw(&Circle {
                      x: bx as f64 + 0.5,
                      y: -by as f64 + 0.5,
                      radius: 0.25,
                      color: Color::Rgb(color.0 * 2, color.1 * 2, color.2 * 2),
                    });
                  }
                  SongButtonAction::ToggleTrackPatterns {
                    track_patterns: _,
                    default,
                    color_off,
                    color_on,
                  } => {
                    let color = match default {
                      true => color_on,
                      false => color_off,
                    };
                    ctx.draw(&Circle {
                      x: bx as f64 + 0.5,
                      y: -by as f64 + 0.5,
                      radius: 0.33,
                      color: Color::Rgb(color.0 * 2, color.1 * 2, color.2 * 2),
                    });
                  }
                  _ => {}
                }
              }
            }
          }
        }

        ctx.draw(&Rectangle {
          x: (self.hovered.0 - self.cam.0) as f64,
          y: -(self.hovered.1 - self.cam.1) as f64,
          width: 1.,
          height: 1.,
          color: Color::White,
        });
      });

    canvas.render(area, buf);
  }
}
