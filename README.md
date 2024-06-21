# Sample gRPC client & server in Rust

This is a simple example to illustrate how to implement a gRPC client and server in Rust.

The protobuf file `proto/helloworld.proto` defines a simple service with one method, `SayHello`, which takes a `HelloRequest` and returns a `HelloReply`. <br>
The build script `build.rs` generates the needed Rust code from the protobuf file. <br>

The server (`src/bin/server`) listens on port 50051 and the client (`src/bin/client`) sends a request to the server waiting for a reply.

## Usage

First, you need to run the server:

```bash
cargo run --bin server
```

Then, you can run the client:

```bash
cargo run --bin client <name>
```
