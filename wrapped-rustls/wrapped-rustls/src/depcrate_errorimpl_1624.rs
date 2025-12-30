// Generated macro for impl_1624 (impl)
macro_rules! Depcrate_errorimpl_1624 {
() => {
// Module: crate::error
// Provides: {"impl_1624"}
// Dependencies: {}
impl RejectedEch { # [doc = " Returns true if the server provided new ECH configurations to use for a fresh retry connection"] # [doc = ""] # [doc = " The `RejectedEch` error can be provided to [`crate::client::EchConfig::for_retry()`]"] # [doc = " to build a new `EchConfig` for a fresh client connection that will use a compatible ECH"] # [doc = " configuration provided by the server for a retry."] pub fn can_retry (& self) -> bool { self . retry_configs . is_some () } # [doc = " Returns an `EchConfigListBytes` with the server's provided retry configurations (if any)"] pub fn retry_configs (& self) -> Option < EchConfigListBytes < 'static > > { let Some (retry_configs) = & self . retry_configs else { return None ; } ; let mut tls_encoded_list = Vec :: new () ; retry_configs . encode (& mut tls_encoded_list) ; Some (EchConfigListBytes :: from (tls_encoded_list)) } }
};
}
