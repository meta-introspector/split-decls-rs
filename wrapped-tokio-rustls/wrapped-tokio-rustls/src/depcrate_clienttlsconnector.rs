// Generated macro for TlsConnector (struct)
macro_rules! Depcrate_clientTlsConnector {
() => {
// Module: crate::client
// Provides: {"TlsConnector"}
// Dependencies: {}
# [doc = " A wrapper around a `rustls::ClientConfig`, providing an async `connect` method."] # [derive (Clone)] pub struct TlsConnector { inner : Arc < ClientConfig > , # [cfg (feature = "early-data")] early_data : bool , }
};
}
