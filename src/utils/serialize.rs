pub fn serialize(key: &str, val: &str) -> anyhow::Result<Vec<u8>> {
    pub mod keev_capnp {
        include!(concat!(env!("OUT_DIR"), "/schema/keev_capnp.rs"));
    }
    use capnp::serialize_packed;
    use keev_capnp::store;
    let mut message = ::capnp::message::Builder::new_default();
    {
        let mut st = message.init_root::<store::Builder>();
        st.set_id(0);
        st.set_key(key);
        st.set_val(val);
    }
    let mut buf = vec![];
    serialize_packed::write_message(&mut buf, &message)?;
    Ok(buf)
}
