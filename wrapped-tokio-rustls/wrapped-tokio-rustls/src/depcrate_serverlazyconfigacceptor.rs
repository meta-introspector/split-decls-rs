// Generated macro for LazyConfigAcceptor (struct)
macro_rules! Depcrate_serverLazyConfigAcceptor {
() => {
// Module: crate::server
// Provides: {"LazyConfigAcceptor"}
// Dependencies: {}
pub struct LazyConfigAcceptor < IO > { acceptor : rustls :: server :: Acceptor , io : Option < IO > , alert : Option < (rustls :: Error , AcceptedAlert) > , }
};
}
