// Generated macro for impl_440 (impl)
macro_rules! Depcrate_quic_connection_errorimpl_440 {
() => {
// Module: crate::quic::connection::error
// Provides: {"impl_440"}
// Dependencies: {}
impl From < HandshakeError > for io :: Error { fn from (err : HandshakeError) -> Self { match err { HandshakeError :: Timeout => Self :: new (io :: ErrorKind :: TimedOut , err) , HandshakeError :: ConnectionClosed => Self :: new (io :: ErrorKind :: NotConnected , err) , } } }
};
}
