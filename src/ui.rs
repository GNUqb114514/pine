//! The rendering module.

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    widgets::{Block, Borders},
};
use ratatui_image::StatefulImage;

use crate::app::App;

/// Render the UI.
pub fn ui<'frame, 'app, 'textarea>(frame: &'frame mut Frame, app: &'app mut App<'textarea>) {
    let chunks = Layout::horizontal(Constraint::from_percentages([50, 50])).split(frame.area());
    let buffer = Block::default().title("Buffer #1").borders(Borders::ALL);
    let textarea = app.textarea();
    frame.render_widget(textarea, buffer.inner(chunks[0]));
    frame.render_widget(buffer, chunks[0]);
    let image = StatefulImage::default();
    let image_buffer = Block::bordered().title("Buffer #2");
    frame.render_stateful_widget(image, image_buffer.inner(chunks[1]), app.image_mut());
    frame.render_widget(image_buffer, chunks[1]);
}
