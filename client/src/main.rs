use clap::Parser;
use pb::hello_service_client::HelloServiceClient;
use std::{error, fs};
use tonic::transport::{Certificate, Channel, ClientTlsConfig};

mod pb {
    tonic::include_proto!("hello");
}

#[derive(Parser)]
struct Args {
    uri: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Args::parse();
    let h2 = args.uri.starts_with("https");
    let mut endpoint = Channel::builder(args.uri.parse()?);

    if h2 {
        let cert = fs::read("cert.pem")?;
        let cert = Certificate::from_pem(cert);
        let tls_config = ClientTlsConfig::new().ca_certificate(cert);
        endpoint = endpoint.tls_config(tls_config)?;
    }

    let channel = endpoint.connect().await?;
    let resp = HelloServiceClient::new(channel).say_hello(()).await?;
    println!("{}", resp.into_inner().msg);
    Ok(())
}
