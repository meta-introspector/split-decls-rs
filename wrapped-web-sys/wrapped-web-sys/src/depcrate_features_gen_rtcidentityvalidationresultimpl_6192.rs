// Generated macro for impl_6192 (impl)
macro_rules! Depcrate_features_gen_RtcIdentityValidationResultimpl_6192 {
() => {
// Module: crate::features::gen_RtcIdentityValidationResult
// Provides: {"impl_6192"}
// Dependencies: {}
impl RtcIdentityValidationResult { # [doc = "Construct a new `RtcIdentityValidationResult`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcIdentityValidationResult`*"] pub fn new (contents : & str , identity : & str) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_contents (contents) ; ret . set_identity (identity) ; ret } # [deprecated = "Use `set_contents()` instead."] pub fn contents (& mut self , val : & str) -> & mut Self { self . set_contents (val) ; self } # [deprecated = "Use `set_identity()` instead."] pub fn identity (& mut self , val : & str) -> & mut Self { self . set_identity (val) ; self } }
};
}
