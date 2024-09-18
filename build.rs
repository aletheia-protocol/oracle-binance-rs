use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("cargo:rerun-if-changed=proto/book_ticker.proto");

    let proto_file = "./proto/book_ticker.proto";  // Zmieniłem na book_ticker.proto
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional") // Obsługa opcji proto3
        .build_client(true)  // Generowanie klienta
        .build_server(true)  // Generowanie serwera
        .file_descriptor_set_path(out_dir.join("book_ticker_descriptor.bin"))  // Zapisuje plik descriptor
        //.out_dir("./src/adapters/proto")  // Wygenerowane pliki będą w src
        .compile(&[proto_file], &["proto"])?;  // Kompilacja plików .proto

    Ok(())
}