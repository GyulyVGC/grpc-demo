use std::sync::{Arc, Mutex};
use std::thread;

use grpc::ServerRequestSingle;
use grpc::ServerResponseUnarySink;
use grpc_test::helloworld::{HelloReply, HelloRequest};
use grpc_test::helloworld_grpc::{Greeter, GreeterServer};
use grpc_test::PORT;

#[derive(Default)]
struct GreeterImpl {
    num_greets: Arc<Mutex<u32>>,
}

impl Greeter for GreeterImpl {
    fn say_hello(
        &self,
        _: grpc::ServerHandlerContext,
        req: ServerRequestSingle<HelloRequest>,
        resp: ServerResponseUnarySink<HelloReply>,
    ) -> grpc::Result<()> {
        let mut r = HelloReply::new();
        let num_greets = *self.num_greets.lock().unwrap();
        *self.num_greets.lock().unwrap() += 1;
        let name = if req.message.get_name().is_empty() {
            "world"
        } else {
            req.message.get_name()
        };
        println!("greeting request from {name} [greet #{num_greets}]");
        r.set_message(format!("Hello {name}"));
        resp.finish(r)
    }
}

fn main() {
    let mut server_builder = grpc::ServerBuilder::new_plain();
    server_builder.http.set_port(PORT);
    server_builder.add_service(GreeterServer::new_service_def(GreeterImpl::default()));

    let server = server_builder.build().expect("server");

    println!("greeter server started on address {}", server.local_addr());

    loop {
        thread::park();
    }
}
