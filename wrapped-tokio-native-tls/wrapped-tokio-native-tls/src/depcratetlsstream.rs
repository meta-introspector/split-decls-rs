// Generated macro for TlsStream (struct)
macro_rules! DepcrateTlsStream {
() => {
// Module: crate
// Provides: {"TlsStream"}
// Dependencies: {}
# [doc = " A wrapper around an underlying raw stream which implements the TLS or SSL"] # [doc = " protocol."] # [doc = ""] # [doc = " A `TlsStream<S>` represents a handshake that has been completed successfully"] # [doc = " and both the server and the client are ready for receiving and sending"] # [doc = " data. Bytes read from a `TlsStream` are decrypted from `S` and bytes written"] # [doc = " to a `TlsStream` are encrypted when passing through to `S`."] # [derive (Debug)] pub struct TlsStream < S > (native_tls :: TlsStream < AllowStd < S > >) ;
};
}
