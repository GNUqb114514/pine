//! The logic of the app.

use std::{error::Error, io};

use ratatui::style::Style;
use ratatui_image::{picker::Picker, protocol::StatefulProtocol};
use tui_textarea::{Input, TextArea};

use crate::{Args, text_buffer::TextBuffer};

/// An app instance.
pub struct App<'textarea> {
    buffer: TextBuffer,
    textarea: TextArea<'textarea>,
    exit: bool,
    image: Option<StatefulProtocol>,
}

impl App<'_> {
    /// Create a new app.
    pub async fn new(args: Args) -> Result<Self, Box<dyn Error>> {
        let buffer = match args.file {
            Some(file_path) => TextBuffer::from_file(file_path).await?,
            None => TextBuffer::new(),
        };
        Ok(Self {
            textarea: {
                let mut textarea = TextArea::from(buffer.lines().map(&str::to_string));
                textarea.set_line_number_style(Style::default());
                textarea.set_cursor_line_style(Style::default());
                textarea
            },
            buffer,
            exit: false,
            image: {
                let picker = Picker::from_query_stdio()?;
                if let Some(img_path) = args.image {
                    let image = image::ImageReader::open(img_path)?.decode()?;
                    Some(picker.new_resize_protocol(image))
                } else {
                    None
                }
            },
        })
    }

    /// Send a input to the textarea.
    ///
    /// This will update the buffer.
    pub fn forward_input(&mut self, input: impl Into<Input>) {
        if self.textarea.input(input) {
            self.buffer
                .set_text(self.textarea.lines().into_iter().cloned().collect());
        }
    }

    /// The text area of the left side.
    pub fn textarea(&'_ self) -> &'_ TextArea<'_> {
        &self.textarea
    }

    /// Whether the app should exit.
    ///
    /// Before exiting, please call [cleanup](Self::cleanup).
    pub fn exit(&self) -> bool {
        self.exit
    }

    /// A mutable reference to the [StatefulProtocol] of the image.
    pub fn image_mut(&mut self) -> Option<&mut StatefulProtocol> {
        self.image.as_mut()
    }

    /// Tell the app that it should exit.
    pub fn set_exit(&mut self) {
        self.exit = true;
    }

    /// Clean up the app.
    ///
    /// This saves all unsaved changes.
    pub async fn cleanup(&mut self) -> io::Result<()> {
        self.buffer.save().await?;
        Ok(())
    }

    /// The buffer of the left side.
    pub fn buffer(&self) -> &TextBuffer {
        &self.buffer
    }

    pub async fn save(&mut self) -> io::Result<()> {
        self.buffer.save().await?;
        Ok(())
    }
}
