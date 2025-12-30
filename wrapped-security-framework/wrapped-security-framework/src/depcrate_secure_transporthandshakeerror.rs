// Generated macro for HandshakeError (enum)
macro_rules! Depcrate_secure_transportHandshakeError {
() => {
// Module: crate::secure_transport
// Provides: {"HandshakeError"}
// Dependencies: {}
# [doc = " An error or intermediate state after a TLS handshake attempt."] # [derive (Debug)] pub enum HandshakeError < S > { # [doc = " The handshake failed."] Failure (Error) , # [doc = " The handshake was interrupted midway through."] Interrupted (MidHandshakeSslStream < S >) , }
};
}
