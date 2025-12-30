// Generated macro for MidHandshakeSslStream (struct)
macro_rules! Depcrate_secure_transportMidHandshakeSslStream {
() => {
// Module: crate::secure_transport
// Provides: {"MidHandshakeSslStream"}
// Dependencies: {}
# [doc = " An SSL stream midway through the handshake process."] # [derive (Debug)] pub struct MidHandshakeSslStream < S > { stream : SslStream < S > , error : Error , }
};
}
