// Generated macro for connect (function)
macro_rules! Depcrate_netconnect {
() => {
// Module: crate::net
// Provides: {"connect"}
// Dependencies: {}
pub fn connect (addr : & SocketAddr) -> std :: io :: Result < Box < dyn Connection > > { match addr { SocketAddr :: Net (addr) => { std :: net :: TcpStream :: connect (addr) . map (| s | Box :: new (s) as Box < dyn Connection >) } # [cfg (unix)] SocketAddr :: Unix (p) => { std :: os :: unix :: net :: UnixStream :: connect (p) . map (| s | Box :: new (s) as Box < dyn Connection >) } # [cfg (any (target_os = "linux" , target_os = "android"))] SocketAddr :: UnixAbstract (p) => { let sock = std :: os :: unix :: net :: SocketAddr :: from_abstract_name (p) ? ; std :: os :: unix :: net :: UnixStream :: connect_addr (& sock) . map (| s | Box :: new (s) as Box < dyn Connection >) } } }
};
}
