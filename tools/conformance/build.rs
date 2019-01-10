use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let mut dest_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    dest_path.push("gen");

    match fs::create_dir_all(&dest_path) {
        Ok(()) => {},
        Err(ref e) if e.kind() == ErrorKind::AlreadyExists => {},
        Err(e) => panic!("{}", e)
    }

    Command::new("../../build/protoc/bin/protoc")
        .arg(format!("--rust_out={}", dest_path.to_str().unwrap()))
        .arg("--proto_path=../../protos")
        .arg("conformance.proto")
        .arg("test_messages_proto2.proto")
        .arg("test_messages_proto3.proto")
        .spawn()
        .expect("failed to run protoc");
}