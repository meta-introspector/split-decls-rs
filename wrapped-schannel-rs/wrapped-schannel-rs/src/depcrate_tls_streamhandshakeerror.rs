// Generated macro for HandshakeError (enum)
macro_rules! Depcrate_tls_streamHandshakeError {
() => {
// Module: crate::tls_stream
// Provides: {"HandshakeError"}
// Dependencies: {}
# [doc = " A failure which can happen during the `Builder::initialize` phase, either an"] # [doc = " I/O error or an intermediate stream which has not completed its handshake."] # [derive (Debug)] pub enum HandshakeError < S > { # [doc = " A fatal I/O error occurred"] Failure (io :: Error) , # [doc = " The stream connection is in progress, but the handshake is not completed"] # [doc = " yet."] Interrupted (MidHandshakeTlsStream < S >) , }
};
}
