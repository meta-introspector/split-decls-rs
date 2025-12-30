// Generated macro for impl_8854 (impl)
macro_rules! Depcrate_features_gen_WorkletOptionsimpl_8854 {
() => {
// Module: crate::features::gen_WorkletOptions
// Provides: {"impl_8854"}
// Dependencies: {}
impl WorkletOptions { # [doc = "Construct a new `WorkletOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `WorkletOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (feature = "RequestCredentials")] # [deprecated = "Use `set_credentials()` instead."] pub fn credentials (& mut self , val : RequestCredentials) -> & mut Self { self . set_credentials (val) ; self } }
};
}
