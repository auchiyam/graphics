use tonic::async_trait;

use proto::{
    greeter_server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};

mod proto {
    use tonic::include_proto;

    include_proto!("helloworld");
}

pub struct Hello;

#[async_trait]
impl Greeter for Hello {
    async fn say_hello(
        &self,
        request: tonic::Request<HelloRequest>,
    ) -> Result<tonic::Response<HelloReply>, tonic::Status> {
        Ok(tonic::Response::new(HelloReply {
            message: "hello world",
        }))
    }
}

impl Hello {
    pub fn service() -> GreeterServer<Hello> {
        GreeterServer::new(Hello)
    }
}
