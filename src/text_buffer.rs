//! The abstraction of text buffers.
use std::{
    io::{self, ErrorKind},
    mem,
    path::{Path, PathBuf},
};

use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt},
};

// Status of a file.
pub enum FileStatus {
    // The file exists in the file system.
    Exist {
        fd: File,
        file_path: PathBuf,
    },
    // The file is not exist on the file system.
    New {
        file_path: PathBuf,
    },
    /// The file is unnamed.
    Unnamed,
}

impl FileStatus {
    /// Write the given text to the file descriptor, overriding all previously exist data.
    pub async fn write(&mut self, text: &str) -> io::Result<()> {
        match self {
            FileStatus::Exist { fd, .. } => {
                fd.write_all(text.as_bytes()).await?;
            }
            FileStatus::New { file_path } => {
                let open_options =
                    mem::take(OpenOptions::new().read(true).write(true).create(true));
                let fd = open_options.open(&file_path).await?;
                let file_path = mem::take(file_path);
                let _ = mem::replace(self, FileStatus::Exist { fd, file_path });
            }
            FileStatus::Unnamed => return Err(io::Error::from(ErrorKind::NotFound)),
        }
        Ok(())
    }

    /// The file path of the buffer, if applicatable.
    pub fn file_path(&self) -> Option<&Path> {
        match self {
            FileStatus::Exist { file_path, .. } | FileStatus::New { file_path } => Some(file_path),
            _ => None,
        }
    }
}

/// A text buffer.
pub struct TextBuffer {
    text: String,
    status: FileStatus,
    dirty: bool,
}

impl TextBuffer {
    /// Create a new empty [unnamed](BufferStatus::Unnamed) buffer.
    pub fn new() -> TextBuffer {
        Self {
            text: String::new(),
            status: FileStatus::Unnamed,
            dirty: false,
        }
    }

    /// Create a new [unnamed](BufferStatus::Unnamed) buffer from a string.
    pub fn from_text(text: String) -> TextBuffer {
        Self {
            text: text,
            status: FileStatus::Unnamed,
            dirty: false,
        }
    }

    /// Create a new buffer assigned with the file.
    ///
    /// This with try to read the file asynchronously and fail if reading failed.
    pub async fn from_file(file_path: PathBuf) -> io::Result<Self> {
        let open_options = mem::take(OpenOptions::new().read(true).write(true));
        let fd = open_options.open(&file_path).await;
        match fd {
            Ok(mut fd) => Ok(Self {
                text: {
                    let mut dst = String::new();
                    fd.read_to_string(&mut dst).await?;
                    dst
                },
                status: FileStatus::Exist { fd, file_path },
                dirty: false,
            }),
            Err(err) => match err.kind() {
                // Extension prepared
                ErrorKind::NotFound => Ok(Self {
                    text: Default::default(),
                    status: FileStatus::New { file_path },
                    dirty: false,
                }),
                _ => return Err(err),
            },
        }
    }

    /// The lines of this buffer.
    pub fn lines(&self) -> impl Iterator<Item = &str> {
        self.text.lines()
    }

    /// Set the text of the buffer.
    ///
    /// Also marks it as dirty.
    pub fn set_text(&mut self, text: String) {
        self.dirty = true;
        self.text = text;
    }

    /// The current status of the buffer.
    pub fn status(&self) -> &FileStatus {
        &self.status
    }

    /// Save the buffer.
    pub async fn save(&mut self) -> io::Result<()> {
        if self.dirty {
            self.status.write(&self.text).await?;
            self.dirty = false;
        }
        Ok(())
    }

    /// The file path of this [`TextBuffer`].
    pub fn file_path(&self) -> Option<&Path> {
        self.status.file_path()
    }

    /// Whether the buffer is dirty.
    pub fn dirty(&self) -> bool {
        self.dirty
    }
}
