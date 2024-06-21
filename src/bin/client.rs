use grpc::{ClientConf, ClientStubExt};

use futures::executor;
use grpc_test::helloworld::HelloRequest;
use grpc_test::helloworld_grpc::GreeterClient;
use grpc_test::PORT;

fn main() {
    let name = std::env::args()
        .nth(1)
        .map_or_else(|| "world".to_owned(), |s| s.clone());

    let client_conf = ClientConf::default();

    let client = GreeterClient::new_plain("::", PORT, client_conf).unwrap();

    let mut req = HelloRequest::new();
    req.set_name(name);

    let resp = client
        .say_hello(grpc::RequestOptions::new(), req)
        .join_metadata_result();

    let hello_reply = executor::block_on(resp).unwrap().1;

    println!("{hello_reply:?}");
}
