use std::fs::File;
use std::io::{Cursor, Seek, SeekFrom};
use std::path::Path;

use crate::color::Color;
use crate::util::UVec2;

const DIB_HEADER_OFFSET: u64 = 14;

pub struct Icon {
    size: UVec2,
    content: Vec<u8>,
}

impl Icon {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Icon, BadIcon> {
        let mut file = File::open(path)?;

        file.seek(SeekFrom::Start(DIB_HEADER_OFFSET));

        todo!()
    }

    pub fn get_size(&self) -> UVec2 {
        self.size
    }

    pub fn draw(&self) -> (Vec<Color>, UVec2) {
        todo!();
    }
}

pub enum BadIcon {
    Io(std::io::Error),
    Bmp(BmpError),
}

impl From<std::io::Error> for BadIcon {
    fn from(value: std::io::Error) -> Self {
        BadIcon::Io(value)
    }
}

impl From<BmpError> for BadIcon {
    fn from(value: BmpError) -> Self {
        BadIcon::Bmp(value)
    }
}

pub enum BmpError {
    BadHeader,
    WrongFormat,
}
