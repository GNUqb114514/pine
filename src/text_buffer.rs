use std::{
    io::{self, ErrorKind},
    mem,
    path::PathBuf,
};

use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt},
};

pub struct TextBuffer {
    text: String,
    file_path: Option<PathBuf>,
    fd: Option<File>,
    dirty: bool,
}

impl TextBuffer {
    pub fn new() -> TextBuffer {
        Self {
            text: String::new(),
            file_path: None,
            fd: None,
            dirty: true,
        }
    }

    pub fn from_text(text: String) -> TextBuffer {
        Self {
            text: text,
            file_path: None,
            fd: None,
            dirty: true,
        }
    }

    pub async fn from_file(file_path: PathBuf) -> io::Result<Self> {
        let open_options = mem::take(OpenOptions::new().read(true).write(true).create(true));
        let fd = open_options.open(&file_path).await;
        let mut fd = match fd {
            Ok(fd) => fd,
            Err(err) => match err.kind() {
                _ => return Err(err),
            },
        };
        Ok(Self {
            file_path: Some(file_path),
            text: {
                let mut dst = String::new();
                fd.read_to_string(&mut dst).await?;
                dst
            },
            fd: Some(fd),
            dirty: false,
        })
    }

    pub fn lines(&self) -> impl Iterator<Item = &str> {
        self.text.lines()
    }

    pub fn set_text(&mut self, text: String) {
        self.dirty = true;
        self.text = text;
    }

    pub fn dirty(&self) -> bool {
        self.dirty
    }

    pub async fn save(&mut self) -> io::Result<()> {
        let Some(fd) = &mut self.fd else {
            return Err(io::Error::from(ErrorKind::NotFound));
        };
        fd.write_all(self.text.as_bytes()).await
    }
}
