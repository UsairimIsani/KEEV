pub fn deserialize(data: &[u8]) -> anyhow::Result<(String, String)> {
    pub mod keev_capnp {
        include!(concat!(env!("OUT_DIR"), "/schema/keev_capnp.rs"));
    }
    use capnp::serialize_packed;
    use keev_capnp::store;
    let message_reader =
        serialize_packed::read_message(data, ::capnp::message::ReaderOptions::new())?;
    let st = message_reader.get_root::<store::Reader>()?;
    Ok((st.get_key()?.to_string(), st.get_val()?.to_string()))
}
