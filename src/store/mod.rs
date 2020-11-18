pub trait Store {
    fn new() -> Self;
    fn get(&mut self, key: &str) -> anyhow::Result<&[u8]>;
    fn set(&mut self, key: &str, val: &str) -> anyhow::Result<&[u8]>;
    fn remove(&mut self, key: &str) -> anyhow::Result<Vec<u8>>;
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
