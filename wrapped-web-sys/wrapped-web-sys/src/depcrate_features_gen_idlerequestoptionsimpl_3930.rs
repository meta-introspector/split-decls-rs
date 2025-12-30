// Generated macro for impl_3930 (impl)
macro_rules! Depcrate_features_gen_IdleRequestOptionsimpl_3930 {
() => {
// Module: crate::features::gen_IdleRequestOptions
// Provides: {"impl_3930"}
// Dependencies: {}
impl IdleRequestOptions { # [doc = "Construct a new `IdleRequestOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `IdleRequestOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_timeout()` instead."] pub fn timeout (& mut self , val : u32) -> & mut Self { self . set_timeout (val) ; self } }
};
}
