use std::{
    fs::{File, OpenOptions},
    io::Result,
};

const PATH: &str = "storage.txt";

pub fn make_file() -> Result<File> {
    OpenOptions::new().read(true).write(true).create(true).open(PATH)
}