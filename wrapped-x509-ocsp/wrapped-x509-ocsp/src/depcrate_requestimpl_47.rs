// Generated macro for impl_47 (impl)
macro_rules! Depcrate_requestimpl_47 {
() => {
// Module: crate::request
// Provides: {"impl_47"}
// Dependencies: {}
impl OcspRequest { # [doc = " Returns the request's nonce value, if any. This method will return `None` if the request"] # [doc = " has no `Nonce` extension or decoding of the `Nonce` extension fails."] pub fn nonce (& self) -> Option < Nonce > { self . tbs_request . nonce () } }
};
}
