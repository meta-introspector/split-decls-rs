// Generated macro for impl_1330 (impl)
macro_rules! Depcrate_net_tcpimpl_1330 {
() => {
// Module: crate::net::tcp
// Provides: {"impl_1330"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a > Iterator for Incoming < 'a > { type Item = io :: Result < TcpStream > ; fn next (& mut self) -> Option < io :: Result < TcpStream > > { Some (self . listener . accept () . map (| p | p . 0)) } }
};
}
