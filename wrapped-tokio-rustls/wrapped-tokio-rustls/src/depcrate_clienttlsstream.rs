// Generated macro for TlsStream (struct)
macro_rules! Depcrate_clientTlsStream {
() => {
// Module: crate::client
// Provides: {"TlsStream"}
// Dependencies: {}
# [doc = " A wrapper around an underlying raw stream which implements the TLS or SSL"] # [doc = " protocol."] # [derive (Debug)] pub struct TlsStream < IO > { pub (crate) io : IO , pub (crate) session : ClientConnection , pub (crate) state : TlsState , pub (crate) need_flush : bool , # [cfg (feature = "early-data")] pub (crate) early_waker : Option < Waker > , }
};
}
