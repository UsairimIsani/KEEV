pub trait Store {
    fn new() -> anyhow::Result<KeevDB>;
    fn get(&mut self, key: &str) -> anyhow::Result<&Vec<u8>>;
    fn set(&mut self, key: &str, val: &str) -> anyhow::Result<&mut Vec<u8>>;
    fn remove(&mut self, key: &str) -> anyhow::Result<Vec<u8>>;
}
pub mod disk;
pub mod memory;
pub use memory::Memory as KeevDB;
