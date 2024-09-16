use async_trait::async_trait;
use clap::Parser;
use core::error;
use pingora::{listeners::TlsSettings, prelude::HttpPeer, protocols::ALPN, server::Server};
use pingora_proxy::{ProxyHttp, Session};

#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = false)]
    tls_termination: bool,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Args::parse();

    let mut server = Server::new(None)?;
    server.bootstrap();

    let mut proxy = pingora_proxy::http_proxy_service(
        &server.configuration,
        GrpcProxy::new(args.tls_termination),
    );

    let mut tls_settings = TlsSettings::intermediate("cert.pem", "key.pem")?;
    tls_settings.set_alpn(ALPN::H2);
    proxy.add_tls_with_settings("[::1]:8443", None, tls_settings);
    server.add_service(proxy);
    server.run_forever();
}

struct GrpcProxy {
    tls_termination: bool,
}

impl GrpcProxy {
    pub fn new(tls_termination: bool) -> Self {
        Self { tls_termination }
    }
}

#[async_trait]
impl ProxyHttp for GrpcProxy {
    type CTX = ();
    fn new_ctx(&self) -> Self::CTX {}

    async fn upstream_peer(
        &self,
        _sesion: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> pingora::Result<Box<HttpPeer>> {
        // if we are terminating TLS, then the upstream peer does not use TLS
        let tls = !self.tls_termination;
        let mut peer = HttpPeer::new(("::1", 50051), tls, String::new());
        peer.options.alpn = ALPN::H2;
        peer.options.verify_cert = false;
        Ok(Box::new(peer))
    }
}
