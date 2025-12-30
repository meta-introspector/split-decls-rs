// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
# [tokio :: main (flavor = "current_thread")] pub async fn main () -> Result < () , Box < dyn std :: error :: Error + Send + Sync > > { env_logger :: Builder :: new () . parse_filters ("debug") . init () ; let addr = SocketAddr :: new (Ipv4Addr :: UNSPECIFIED . into () , 9975) ; let listener = TcpListener :: bind (addr) . await ? ; println ! ("Listening on http://{addr}") ; loop { let (tcp , _) = listener . accept () . await ? ; let io = TokioIo :: new (tcp) ; tokio :: task :: spawn (async move { if let Err (err) = http1 :: Builder :: new () . serve_connection (io , service_fn (hello)) . await { println ! ("Error serving connection: {err:?}") ; } }) ; } }
};
}
