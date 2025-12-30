// Generated macro for Accept (struct)
macro_rules! Depcrate_serverAccept {
() => {
// Module: crate::server
// Provides: {"Accept"}
// Dependencies: {}
# [doc = " Future returned from `TlsAcceptor::accept` which will resolve"] # [doc = " once the accept handshake has finished."] pub struct Accept < IO > (MidHandshake < TlsStream < IO > >) ;
};
}
