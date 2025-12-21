use std::path::PathBuf;

use clap::Parser;
use mime::Mime;

#[derive(Debug, Clone)]
pub enum FileType {
    Text,
    Image,
}

#[derive(Debug, Clone)]
pub struct File {
    path: PathBuf,
    file_type: FileType,
    mime: Mime,
}

#[derive(Debug, Parser)]
/// CLI Argument.
pub struct Cli {
    pub files: Vec<PathBuf>,
}
