use clap::Parser;
use pb::{
    hello_service_server::{HelloService, HelloServiceServer},
    HelloResponse,
};
use std::{
    error, fs,
    net::{Ipv6Addr, SocketAddr},
};
use tonic::{
    transport::{Identity, ServerTlsConfig},
    Request, Response, Status,
};

mod pb {
    tonic::include_proto!("hello");
}

#[derive(Debug)]
struct MyHelloService;

#[tonic::async_trait]
impl HelloService for MyHelloService {
    async fn say_hello(&self, _: Request<()>) -> Result<Response<HelloResponse>, Status> {
        Ok(Response::new(HelloResponse {
            msg: "Hello".to_owned(),
        }))
    }
}

#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = false)]
    tls: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Args::parse();
    let addr = SocketAddr::new(Ipv6Addr::LOCALHOST.into(), 50051);
    let mut server = tonic::transport::Server::builder();

    if args.tls {
        let cert = fs::read("cert.pem")?;
        let key = fs::read("key.pem")?;
        let identity = Identity::from_pem(cert, key);
        let tls_config = ServerTlsConfig::new().identity(identity);
        server = server.tls_config(tls_config)?;
    }

    server
        .add_service(HelloServiceServer::new(MyHelloService))
        .serve(addr)
        .await?;
    Ok(())
}
