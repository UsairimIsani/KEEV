use std::{
    fs::{File, OpenOptions},
    io::{prelude::*, BufWriter},
    time::SystemTime,
};
pub trait Store {
    fn new() -> Self;
    fn get(&mut self, key: &str) -> anyhow::Result<&[u8]>;
    fn set(&mut self, key: &str, val: &str) -> anyhow::Result<&[u8]>;
    fn remove(&mut self, key: &str) -> anyhow::Result<Vec<u8>>;
    fn log(name: &str, key: &str, val: &str, db: &str) {
        println!(
            "{} {} {} {} {}",
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            name,
            key,
            val,
            db
        );
    }
}
pub enum DBType {
    InMemory,
    InDisk,
    Persistant,
}
pub fn new<T: Store>(db_type: DBType) -> T {
    // For Future Impl
    // db_type
    T::new()
}
pub mod db;
pub mod disk;
pub mod memory;
pub use db::DB as KeevDB;
