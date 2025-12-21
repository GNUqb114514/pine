//! Pine Editor, a TUI text editor.
//!
//! See [the repo page](https://github.com/GNUqb114514/pine) for detail.

mod app;
mod text_buffer;
mod ui;

use std::{error::Error, path::PathBuf};

use clap::Parser;
use crossterm::event::EventStream;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use tokio_stream::StreamExt;

use crate::{app::App, ui::ui};

#[derive(Parser)]
/// CLI Argument.
pub struct Args {
    /// The path of the file opened on the right side.
    #[arg(short, long)]
    pub file: Option<PathBuf>,
    /// The path of the image shown on the right side.
    #[arg(short, long)]
    pub image: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut terminal = ratatui::init();
    let mut app = App::new(args).await?;
    let mut events = EventStream::new();
    loop {
        terminal.draw(|f| {
            ui(f, &mut app);
        })?;

        let Some(next) = events.next().await else {
            continue;
        };

        if let Event::Key(
            key @ KeyEvent {
                kind: event::KeyEventKind::Press,
                ..
            },
        ) = next?
        {
            match key.code {
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.set_exit();
                }
                KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.save().await?;
                }
                _ => {
                    app.forward_input(key);
                }
            }
        };

        if app.exit() {
            app.cleanup().await?;
            break;
        }
    }
    ratatui::restore();
    Ok(())
}
