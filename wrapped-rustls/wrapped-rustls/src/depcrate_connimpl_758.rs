// Generated macro for impl_758 (impl)
macro_rules! Depcrate_connimpl_758 {
() => {
// Module: crate::conn
// Provides: {"impl_758"}
// Dependencies: {}
impl < Side : SideData > UnbufferedConnectionCommon < Side > { # [doc = " Extract secrets, so they can be used when configuring kTLS, for example."] # [doc = " Should be used with care as it exposes secret key material."] pub fn dangerous_extract_secrets (self) -> Result < ExtractedSecrets , Error > { self . core . dangerous_extract_secrets () } }
};
}
