use anyhow::Result;
use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
  layout::Margin,
  symbols::border,
  widgets::{Block, Widget},
  DefaultTerminal, Frame,
};
use sophixer_core::song_data::Set;

use crate::views::{select::ViewSelect, View};

mod views;

pub struct EditorData {
  set: Set,
}

pub struct Editor {
  exit: bool,
  data_path: String,
  data: EditorData,
  current_view: Box<dyn View>,
}

impl Widget for &Editor {
  fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
  where
    Self: Sized,
  {
    let block = Block::bordered()
      .title(self.current_view.get_title())
      .title_bottom(self.current_view.get_instructions())
      .border_set(border::THICK);
    let inner_area = block.inner(area).inner(Margin::new(1, 1));

    self.current_view.render(&self.data, inner_area, buf);

    block.render(area, buf);
  }
}

impl Editor {
  pub fn new(data_folder: String) -> Result<Self> {
    let s = Self {
      exit: false,
      data_path: data_folder.clone(),
      data: EditorData {
        set: Set::from_folder(data_folder)?,
      },
      current_view: Box::new(ViewSelect::new()?),
    };

    Ok(s)
  }

  pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
    while !self.exit {
      terminal.draw(|frame| self.draw(frame))?;
      self.handle_events()?;
    }
    ratatui::restore();
    Ok(())
  }

  fn draw(&self, frame: &mut Frame) {
    frame.render_widget(self, frame.area());
  }

  fn handle_events(&mut self) -> Result<()> {
    match event::read()? {
      Event::Key(ke) if ke.kind == KeyEventKind::Press => {
        self.handle_key_event(ke)?;
      }
      _ => {}
    }
    Ok(())
  }

  fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
    match key_event.code {
      KeyCode::Char('q') => {
        self.exit = true;
        Ok(())
      }
      KeyCode::Char('s') => {
        self.data.set.save_in_folder(self.data_path.clone())?;
        Ok(())
      }
      _ => {
        if let Some(n) = self
          .current_view
          .handle_key_event(&mut self.data, key_event)?
        {
          self.current_view = n;
        }
        Ok(())
      }
    }
  }
}

#[derive(Parser, Debug)]
struct Args {
  data_path: String,
}

fn main() -> Result<()> {
  let args = Args::parse();

  ratatui::run(|terminal| Editor::new(args.data_path)?.run(terminal))
}
