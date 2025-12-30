// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
# [tokio :: main (flavor = "current_thread")] pub async fn main () -> Result < () , Box < dyn std :: error :: Error + Send + Sync > > { env_logger :: Builder :: new () . parse_filters ("debug") . init () ; println ! ("USE THIS AT YOUR OWN RISK!") ; println ! ("See https://github.com/RustCrypto/rustls-rustcrypto for details!") ; let addr = SocketAddr :: new (Ipv4Addr :: UNSPECIFIED . into () , 9975) ; let certs = load_certs () ? ; let key = load_private_key () ? ; println ! ("Starting to serve on https://{addr}") ; let incoming = TcpListener :: bind (addr) . await ? ; let mut server_config = ServerConfig :: builder_with_provider (Arc :: new (rustls_rustcrypto :: provider ())) . with_safe_default_protocol_versions () ? . with_no_client_auth () . with_single_cert (certs , key) ? ; server_config . alpn_protocols = vec ! [b"h2" . to_vec () , b"http/1.1" . to_vec () , b"http/1.0" . to_vec ()] ; let tls_acceptor = TlsAcceptor :: from (Arc :: new (server_config)) ; let service = service_fn (echo) ; loop { let (tcp_stream , _remote_addr) = incoming . accept () . await ? ; let tls_acceptor = tls_acceptor . clone () ; tokio :: spawn (async move { let tls_stream = match tls_acceptor . accept (tcp_stream) . await { Ok (tls_stream) => tls_stream , Err (err) => { eprintln ! ("failed to perform tls handshake: {err:#}") ; return ; } } ; if let Err (_err) = Builder :: new (TokioExecutor :: new ()) . serve_connection (TokioIo :: new (tls_stream) , service) . await { } }) ; } }
};
}
