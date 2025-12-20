mod app;
mod ui;

use std::{error::Error, path::PathBuf};

use clap::Parser;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

use crate::{app::App, ui::ui};

#[derive(Parser)]
pub struct Args {
    pub img_path: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut terminal = ratatui::init();
    let mut app = App::new(args)?;
    loop {
        terminal.draw(|f| {
            ui(f, &mut app);
        })?;

        if let Event::Key(
            key @ KeyEvent {
                kind: event::KeyEventKind::Press,
                ..
            },
        ) = event::read()?
        {
            match key.code {
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.set_exit()
                }
                _ => {
                    app.forward_input(key);
                }
            }
        };

        if app.exit() {
            break;
        }
    }
    ratatui::restore();
    Ok(())
}
