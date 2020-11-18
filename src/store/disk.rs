//! Need to Rewrite
use super::Store;
use crate::utils::serialize;
use std::{
    fs::{File, OpenOptions},
    io::{prelude::*, BufWriter},
};

pub struct Disk {
    file: BufWriter<File>,
}
impl Store for Disk {
    fn new() -> Self {
        let f = OpenOptions::new()
            .append(true)
            .create(true)
            .open("keev.db")
            .expect("Couldn't open file");
        Self {
            file: BufWriter::new(f),
        }
    }
    fn get(&mut self, key: &str) -> anyhow::Result<&[u8]> {
        Ok(self.file.buffer()) // Need to FIx the Disk impl in everyway!
    }
    fn set(&mut self, key: &str, val: &str) -> anyhow::Result<&[u8]> {
        let data = serialize(key, val)?;
        self.file.write(&data)?; // Need to fix this hack
        Ok(&[])
    }
    fn remove(&mut self, key: &str) -> anyhow::Result<Vec<u8>> {
        let r = self.file.buffer().to_vec();
        // TODO remove the entry from the file
        Ok(r)
    }
}
