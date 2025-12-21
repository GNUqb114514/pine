//! The rendering module.

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    widgets::{Block, Borders},
};
use ratatui_image::StatefulImage;

use crate::{app::App, text_buffer::FileStatus};

/// Render the UI.
pub fn ui<'frame, 'app, 'textarea>(frame: &'frame mut Frame, app: &'app mut App<'textarea>) {
    let chunks = Layout::horizontal(Constraint::from_percentages([50, 50])).split(frame.area());

    let buffer = Block::default()
        .title("Buffer #1")
        .borders(Borders::ALL)
        .title_bottom(format!(
            "{}{}",
            app.buffer()
                .file_path()
                .map_or(std::borrow::Cow::Borrowed("UNNAMED"), |val| {
                    val.to_string_lossy()
                }),
            if !app.buffer().dirty() {
                ""
            } else {
                match app.buffer().status() {
                    FileStatus::Exist { .. } => "+",
                    FileStatus::New { .. } | FileStatus::Unnamed { .. } => "*",
                }
            }
        ));
    let textarea = app.textarea();
    frame.render_widget(textarea, buffer.inner(chunks[0]));
    frame.render_widget(buffer, chunks[0]);

    let image = StatefulImage::default();
    let image_buffer = Block::bordered().title("Buffer #2");
    if let Some(image_protocol) = app.image_mut() {
        frame.render_stateful_widget(image, image_buffer.inner(chunks[1]), image_protocol);
    }
    frame.render_widget(image_buffer, chunks[1]);
}
