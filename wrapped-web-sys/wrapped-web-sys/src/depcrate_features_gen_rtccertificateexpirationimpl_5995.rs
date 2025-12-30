// Generated macro for impl_5995 (impl)
macro_rules! Depcrate_features_gen_RtcCertificateExpirationimpl_5995 {
() => {
// Module: crate::features::gen_RtcCertificateExpiration
// Provides: {"impl_5995"}
// Dependencies: {}
impl RtcCertificateExpiration { # [doc = "Construct a new `RtcCertificateExpiration`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcCertificateExpiration`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_expires()` instead."] pub fn expires (& mut self , val : f64) -> & mut Self { self . set_expires (val) ; self } }
};
}
