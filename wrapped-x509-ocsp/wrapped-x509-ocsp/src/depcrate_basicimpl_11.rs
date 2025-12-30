// Generated macro for impl_11 (impl)
macro_rules! Depcrate_basicimpl_11 {
() => {
// Module: crate::basic
// Provides: {"impl_11"}
// Dependencies: {}
impl BasicOcspResponse { # [doc = " Returns the response's nonce value, if any. This method will return `None` if the response"] # [doc = " has no `Nonce` extension or decoding of the `Nonce` extension fails."] pub fn nonce (& self) -> Option < Nonce > { self . tbs_response_data . nonce () } }
};
}
