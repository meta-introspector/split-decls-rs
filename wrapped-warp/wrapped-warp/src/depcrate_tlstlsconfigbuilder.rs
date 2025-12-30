// Generated macro for TlsConfigBuilder (struct)
macro_rules! Depcrate_tlsTlsConfigBuilder {
() => {
// Module: crate::tls
// Provides: {"TlsConfigBuilder"}
// Dependencies: {}
# [doc = " Builder to set the configuration for the Tls server."] pub (crate) struct TlsConfigBuilder { cert : Box < dyn Read + Send + Sync > , key : Box < dyn Read + Send + Sync > , client_auth : TlsClientAuth , ocsp_resp : Vec < u8 > , }
};
}
