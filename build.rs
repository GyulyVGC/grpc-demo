fn main() {
    protoc_rust_grpc::Codegen::new()
        .out_dir("src")
        .include("proto")
        .inputs(["proto/helloworld.proto"])
        .rust_protobuf(true)
        .run()
        .expect("protoc-rust-grpc");
}
