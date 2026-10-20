fn main() {
    // Same prost configuration as f1r3node-rust/models/build.rs for the
    // settings that affect encode/decode cost: BTreeMap maps, Bytes for
    // .casper / .routing bytes fields.
    prost_build::Config::new()
        .btree_map(["."])
        .bytes([".casper", ".routing"])
        .type_attribute(".rhoapi", "#[derive(serde::Serialize)]")
        .compile_protos(&["proto/CasperMessage.proto", "proto/RhoTypes.proto"], &["proto"])
        .expect("prost build");

    capnpc::CompilerCommand::new()
        .src_prefix("schema")
        .file("schema/f1r3.capnp")
        .run()
        .expect("capnp compile");
}
