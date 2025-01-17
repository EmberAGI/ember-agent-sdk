use std::process::Command;

fn main() {
    tonic_build::configure()
        .build_server(false)
        .out_dir("src/")
        .compile_protos(&["schema.proto"], &["./"])
        .unwrap();

    Command::new("cargo").args(["fmt"]).output().unwrap();
}
