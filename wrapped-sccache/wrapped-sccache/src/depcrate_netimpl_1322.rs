// Generated macro for impl_1322 (impl)
macro_rules! Depcrate_netimpl_1322 {
() => {
// Module: crate::net
// Provides: {"impl_1322"}
// Dependencies: {}
impl Acceptor for tokio :: net :: TcpListener { type Socket = tokio :: net :: TcpStream ; # [inline] fn accept (& self) -> impl Future < Output = tokio :: io :: Result < Self :: Socket > > + Send { tokio :: net :: TcpListener :: accept (self) . and_then (| (s , _) | futures :: future :: ok (s)) } # [inline] fn local_addr (& self) -> tokio :: io :: Result < Option < SocketAddr > > { tokio :: net :: TcpListener :: local_addr (self) . map (| a | Some (SocketAddr :: Net (a))) } }
};
}
