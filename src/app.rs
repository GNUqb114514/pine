use std::error::Error;

use ratatui::style::Style;
use ratatui_image::{picker::Picker, protocol::StatefulProtocol};
use tui_textarea::{Input, TextArea};

use crate::Args;

pub struct App<'textarea> {
    textarea: TextArea<'textarea>,
    exit: bool,
    image: StatefulProtocol,
}

impl App<'_> {
    pub fn new(args: Args) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            textarea: {
                let mut textarea = TextArea::default();
                textarea.set_line_number_style(Style::default());
                textarea.set_cursor_line_style(Style::default());
                textarea
            },
            exit: false,
            image: {
                let picker = Picker::from_query_stdio()?;
                let image = image::ImageReader::open(args.img_path)?.decode()?;
                picker.new_resize_protocol(image)
            },
        })
    }
    pub fn forward_input(&mut self, input: impl Into<Input>) {
        self.textarea.input(input);
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
}
