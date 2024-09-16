# pingora-grpc-example

This repo contains a Pingora gRPC example with and without TLS termination.

```
openssl req -x509 -newkey rsa:2048 -keyout key.pem -out cert.pem -days 365 -nodes -subj "/CN=hello.local" \
    -addext "subjectAltName=DNS:hello.local" \
    -addext "basicConstraints=CA:FALSE"
```

Rustls is strict. It will not work with an IP address like _https://[::1]_.

We map _[::1]_ to _hello.local_ in DNS. One way to do this is to add this line in _/etc/hosts_:

```
::1 hello.local
```

## Usage

Proxy handling TLS:

```
proxy --tls-termination
server
client http://hello.local:50051
client https://hello.local:8443
```

Tonic handling TLS:

```
proxy
server --tls
client https://hello.local:50051
client https://hello.local:8443
```
