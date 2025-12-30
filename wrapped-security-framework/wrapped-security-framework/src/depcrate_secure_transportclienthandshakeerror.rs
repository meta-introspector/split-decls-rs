// Generated macro for ClientHandshakeError (enum)
macro_rules! Depcrate_secure_transportClientHandshakeError {
() => {
// Module: crate::secure_transport
// Provides: {"ClientHandshakeError"}
// Dependencies: {}
# [doc = " An error or intermediate state after a TLS handshake attempt."] # [derive (Debug)] pub enum ClientHandshakeError < S > { # [doc = " The handshake failed."] Failure (Error) , # [doc = " The handshake was interrupted midway through."] Interrupted (MidHandshakeClientBuilder < S >) , }
};
}
