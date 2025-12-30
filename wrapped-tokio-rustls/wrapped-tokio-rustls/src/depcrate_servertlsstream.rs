// Generated macro for TlsStream (struct)
macro_rules! Depcrate_serverTlsStream {
() => {
// Module: crate::server
// Provides: {"TlsStream"}
// Dependencies: {}
# [doc = " A wrapper around an underlying raw stream which implements the TLS or SSL"] # [doc = " protocol."] # [derive (Debug)] pub struct TlsStream < IO > { pub (crate) io : IO , pub (crate) session : ServerConnection , pub (crate) state : TlsState , pub (crate) need_flush : bool , }
};
}
