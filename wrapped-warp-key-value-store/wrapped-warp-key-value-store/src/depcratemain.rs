// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
# [tokio :: main] async fn main () { tracing_subscriber :: fmt :: init () ; let config = Config :: parse () ; let addr = SocketAddr :: from (([0 , 0 , 0 , 0] , config . port)) ; let listener = TcpListener :: bind (addr) . await . unwrap () ; serve_forever (listener) . await . expect ("server error") ; }
};
}
