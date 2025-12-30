// Generated macro for impl_5847 (impl)
macro_rules! Depcrate_features_gen_RegistrationOptionsimpl_5847 {
() => {
// Module: crate::features::gen_RegistrationOptions
// Provides: {"impl_5847"}
// Dependencies: {}
impl RegistrationOptions { # [doc = "Construct a new `RegistrationOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RegistrationOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_scope()` instead."] pub fn scope (& mut self , val : & str) -> & mut Self { self . set_scope (val) ; self } # [deprecated = "Use `set_type()` instead."] pub fn type_ (& mut self , val : & str) -> & mut Self { self . set_type (val) ; self } # [cfg (feature = "ServiceWorkerUpdateViaCache")] # [deprecated = "Use `set_update_via_cache()` instead."] pub fn update_via_cache (& mut self , val : ServiceWorkerUpdateViaCache) -> & mut Self { self . set_update_via_cache (val) ; self } }
};
}
