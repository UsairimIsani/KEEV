use capnp::serialize::{read_message, write_message};
use std::collections::btree_map::BTreeMap;

/// Use Capnp to serialize data and save into memory as bytes.
///
/// Only deserialize when `get` requested

pub trait Store {
    fn get(&mut self, key: &str) -> anyhow::Result<&String>;
    fn set();
    fn remove();
    fn insert();
    // Should iterate
}
