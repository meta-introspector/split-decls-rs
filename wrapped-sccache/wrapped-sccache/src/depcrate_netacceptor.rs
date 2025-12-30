// Generated macro for Acceptor (trait)
macro_rules! Depcrate_netAcceptor {
() => {
// Module: crate::net
// Provides: {"Acceptor"}
// Dependencies: {}
pub trait Acceptor { type Socket : AsyncRead + AsyncWrite + Unpin + Send ; fn accept (& self) -> impl Future < Output = tokio :: io :: Result < Self :: Socket > > + Send ; fn local_addr (& self) -> tokio :: io :: Result < Option < SocketAddr > > ; }
};
}
