use anyhow::Result;
use clap::{arg, value_parser, Command};
use tin_core::song_data::{Set, Song, SongButton, SongButtonAction, SongSection};

fn cli() -> Command {
  Command::new("sophixer-editor")
    .about("Sophixer's song editor")
    .subcommand_required(true)
    .arg_required_else_help(true)
    .subcommand(
      Command::new("new")
        .about("Creates a new set")
        .arg(arg!(<PATH> "Path for the new folder"))
        .arg(arg!(<NAME> "Name of the new set"))
        .arg(arg!(<AUTHORS> "Authors of the new set")),
    )
    .subcommand(
      Command::new("edit")
        .about("Edit a set")
        .arg(arg!(<PATH> "Path to the set folder"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
          Command::new("song")
            .about("Manage songs")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(
              Command::new("new")
                .about("Create new song")
                .arg(arg!(<ID> "ID for the new song"))
                .arg(arg!(<SONGPATH> "Path for the new song"))
                .arg(arg!(<NAME> "Name for the new song"))
                .arg(arg!(<AUTHORS> "Authors for the new song")),
            ),
        )
        .subcommand(
          Command::new("section")
            .about("Manage sections in songs")
            .arg(arg!(<ID> "Song ID"))
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(
              Command::new("new")
                .about("Create new section")
                .arg(arg!(<POS> "Position of the new section").value_parser(value_parser!(i64))),
            ),
        )
        .subcommand(
          Command::new("button")
            .about("Manage buttons in sections")
            .arg(arg!(<ID> "Song ID"))
            .arg(arg!(<POS> "Section pos").value_parser(value_parser!(i64)))
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(
              Command::new("new")
                .about("Create new button")
                .arg(arg!(<BUTTONPOS> "Button pos").value_parser(value_parser!(i64)))
                .arg(
                  arg!(<TYPE> "Type of the button")
                    .value_parser(["toggle_channels", "toggle_track_patterns"]),
                ),
            ),
        ),
    )
}

pub fn main() -> Result<()> {
  let matches = cli().get_matches();

  match matches.subcommand() {
    Some(("new", matches)) => {
      let path = matches
        .get_one::<String>("PATH")
        .ok_or(anyhow::Error::msg("required"))?;
      let name = matches
        .get_one::<String>("NAME")
        .ok_or(anyhow::Error::msg("required"))?;
      let authors = matches
        .get_one::<String>("AUTHORS")
        .ok_or(anyhow::Error::msg("required"))?;

      let set = Set::new(name.clone(), authors.clone())?;
      set.save_in_folder(path.clone())?;

      println!("New set created at {}", path);
      Ok(())
    }
    Some(("edit", matches)) => {
      let path = matches
        .get_one::<String>("PATH")
        .ok_or(anyhow::Error::msg("required"))?;
      let mut set = Set::from_folder(path.clone())?;

      match matches.subcommand() {
        Some(("song", matches)) => match matches.subcommand() {
          Some(("new", matches)) => {
            let song_id = matches
              .get_one::<String>("ID")
              .ok_or(anyhow::Error::msg("required"))?;
            let song_path = matches
              .get_one::<String>("SONGPATH")
              .ok_or(anyhow::Error::msg("required"))?;
            let name = matches
              .get_one::<String>("NAME")
              .ok_or(anyhow::Error::msg("required"))?;
            let authors = matches
              .get_one::<String>("AUTHORS")
              .ok_or(anyhow::Error::msg("required"))?;

            set.songs.insert(
              song_id.clone(),
              Song::new(name.clone(), authors.clone(), song_path.clone())?,
            );
            set.save_in_folder(path.clone())?;

            Ok(())
          }
          _ => unreachable!(),
        },
        Some(("section", matches)) => {
          let song_id = matches
            .get_one::<String>("ID")
            .ok_or(anyhow::Error::msg("required"))?;
          let song = set
            .songs
            .get_mut(song_id)
            .ok_or(anyhow::Error::msg("song not found"))?;
          match matches.subcommand() {
            Some(("new", matches)) => {
              let pos = matches
                .get_one::<i64>("POS")
                .ok_or(anyhow::Error::msg("required"))?;

              if !song.sections.contains_key(pos) {
                song.sections.insert(pos.clone(), SongSection::default());
              }
              set.save_in_folder(path.clone())?;

              Ok(())
            }
            _ => unreachable!(),
          }
        }
        Some(("button", matches)) => {
          let song_id = matches
            .get_one::<String>("ID")
            .ok_or(anyhow::Error::msg("required"))?;
          let song = set
            .songs
            .get_mut(song_id)
            .ok_or(anyhow::Error::msg("song not found"))?;
          let section_pos = matches
            .get_one::<i64>("POS")
            .ok_or(anyhow::Error::msg("required"))?;
          let section = song
            .sections
            .get_mut(section_pos)
            .ok_or(anyhow::Error::msg("section not found"))?;

          match matches.subcommand() {
            Some(("new", matches)) => {
              let button_pos = matches
                .get_one::<i64>("BUTTONPOS")
                .ok_or(anyhow::Error::msg("required"))?;
              let typ = matches
                .get_one::<String>("TYPE")
                .ok_or(anyhow::Error::msg("required"))?;

              if !section.buttons.contains_key(button_pos) {
                match typ.as_str() {
                  "toggle_channels" => {
                    section.buttons.insert(
                      button_pos.clone(),
                      SongButton::new(SongButtonAction::default_toggle_channels()?)?,
                    );
                  }
                  "toggle_track_patterns" => {
                    section.buttons.insert(
                      button_pos.clone(),
                      SongButton::new(SongButtonAction::default_toggle_track_patterns()?)?,
                    );
                  }
                  _ => unreachable!(),
                };
              }

              set.save_in_folder(path.clone())?;

              Ok(())
            }
            _ => unreachable!(),
          }
        }
        _ => unreachable!(),
      }
    }
    _ => unreachable!(),
  }
}
