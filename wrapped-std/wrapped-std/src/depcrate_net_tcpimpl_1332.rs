// Generated macro for impl_1332 (impl)
macro_rules! Depcrate_net_tcpimpl_1332 {
() => {
// Module: crate::net::tcp
// Provides: {"impl_1332"}
// Dependencies: {}
# [unstable (feature = "tcplistener_into_incoming" , issue = "88373")] impl Iterator for IntoIncoming { type Item = io :: Result < TcpStream > ; fn next (& mut self) -> Option < io :: Result < TcpStream > > { Some (self . listener . accept () . map (| p | p . 0)) } }
};
}
