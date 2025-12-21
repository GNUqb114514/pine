use std::{error::Error, io};

use ratatui::style::Style;
use ratatui_image::{picker::Picker, protocol::StatefulProtocol};
use tui_textarea::{Input, TextArea};

use crate::{Args, text_buffer::TextBuffer};

pub struct App<'textarea> {
    buffer: TextBuffer,
    textarea: TextArea<'textarea>,
    exit: bool,
    image: StatefulProtocol,
}

impl App<'_> {
    pub async fn new(args: Args) -> Result<Self, Box<dyn Error>> {
        let buffer = TextBuffer::from_file(args.file_path).await?;
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
                let image = image::ImageReader::open(args.img_path)?.decode()?;
                picker.new_resize_protocol(image)
            },
        })
    }
    pub fn forward_input(&mut self, input: impl Into<Input>) {
        if self.textarea.input(input) {
            self.buffer
                .set_text(self.textarea.lines().into_iter().cloned().collect());
        }
    }
    pub fn textarea(&'_ self) -> &'_ TextArea<'_> {
        &self.textarea
    }
    pub fn exit(&self) -> bool {
        self.exit
    }
    pub fn image_mut(&mut self) -> &mut StatefulProtocol {
        &mut self.image
    }
    pub fn set_exit(&mut self) {
        self.exit = true;
    }

    pub async fn cleanup(&mut self) -> io::Result<()> {
        if self.buffer.dirty() {
            self.buffer.save().await?;
        }
        Ok(())
    }
}
