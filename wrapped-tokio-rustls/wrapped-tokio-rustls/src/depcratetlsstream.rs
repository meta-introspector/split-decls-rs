// Generated macro for TlsStream (enum)
macro_rules! DepcrateTlsStream {
() => {
// Module: crate
// Provides: {"TlsStream"}
// Dependencies: {}
# [doc = " Unified TLS stream type"] # [doc = ""] # [doc = " This abstracts over the inner `client::TlsStream` and `server::TlsStream`, so you can use"] # [doc = " a single type to keep both client- and server-initiated TLS-encrypted connections."] # [allow (clippy :: large_enum_variant)] # [derive (Debug)] pub enum TlsStream < T > { Client (client :: TlsStream < T >) , Server (server :: TlsStream < T >) , }
};
}
