fn main() {
    ::capnpc::CompilerCommand::new()
        .file("schema/keev.capnp")
        .run()
        .expect("compiling schema");
}
