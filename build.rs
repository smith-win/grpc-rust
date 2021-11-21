use std::path::{Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // this puts in default place or that specified by "out-dir"
    tonic_build::compile_protos("protos/service.proto")?;


    // let proto_path: &Path = "protos/service.proto".as_ref();
    // // directory the main .proto file resides in
    // let proto_dir = proto_path
    //     .parent()
    //     .expect("proto file should reside in a directory");

    // tonic_build::configure()
    //     .out_dir("target/tonic_build")
    //     .compile( &[proto_path], &[proto_dir])?;

    //
    Ok(())
}