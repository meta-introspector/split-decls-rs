// Generated macro for resolve_socket_addr (function)
macro_rules! Depcrate_net_socket_addrresolve_socket_addr {
() => {
// Module: crate::net::socket_addr
// Provides: {"resolve_socket_addr"}
// Dependencies: {}
fn resolve_socket_addr (lh : LookupHost) -> io :: Result < vec :: IntoIter < SocketAddr > > { let p = lh . port () ; let v : Vec < _ > = lh . map (| mut a | { a . set_port (p) ; a }) . collect () ; Ok (v . into_iter ()) }
};
}
