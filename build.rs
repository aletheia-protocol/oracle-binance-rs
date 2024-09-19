use std::{env, fs};
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("cargo:rerun-if-changed=proto/book_ticker.proto");

    let proto_files = &[
        "./proto/book_ticker.proto",
        "./proto/order_book.proto",
        "./proto/trade.proto"];
    let proto_include = &["proto"];

    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional") // Obsługa opcji proto3
        .build_client(true)  // Generowanie klienta
        .build_server(true)  // Generowanie serwera
        //.out_dir("./src/adapters/proto")  // Wygenerowane pliki będą w src
        .compile(proto_files, proto_include)?;

    // Get OUT_DIR where prost generated the files
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    // Define your custom destination path
    let custom_out_dir = Path::new("./src/adapters/proto");

    // Copy the generated files to your custom path
    for entry in fs::read_dir(out_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            let file_name = entry.file_name();
            fs::copy(&path, custom_out_dir.join(file_name))?;
        }
    }

    Ok(())
}