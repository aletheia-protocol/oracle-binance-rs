fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("cargo:rerun-if-changed=proto/book_ticker.proto");

    let proto_files = &["./proto/book_ticker.proto", "./proto/order_book.proto"];
    let proto_include = &["proto"];

    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional") // Obsługa opcji proto3
        .build_client(true)  // Generowanie klienta
        .build_server(true)  // Generowanie serwera
        //.out_dir("./src/adapters/proto")  // Wygenerowane pliki będą w src
        .compile(proto_files, proto_include)?;

    Ok(())
}