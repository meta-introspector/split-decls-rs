// Generated macro for impl_638 (impl)
macro_rules! Depcrate_secure_transportimpl_638 {
() => {
// Module: crate::secure_transport
// Provides: {"impl_638"}
// Dependencies: {}
impl SessionState { # [doc = " The session has been aborted due to an error."] pub const ABORTED : Self = Self (kSSLAborted) ; # [doc = " The session has been terminated."] pub const CLOSED : Self = Self (kSSLClosed) ; # [doc = " The session is connected."] pub const CONNECTED : Self = Self (kSSLConnected) ; # [doc = " The session is in the handshake process."] pub const HANDSHAKE : Self = Self (kSSLHandshake) ; # [doc = " The session has not yet started."] pub const IDLE : Self = Self (kSSLIdle) ; }
};
}
