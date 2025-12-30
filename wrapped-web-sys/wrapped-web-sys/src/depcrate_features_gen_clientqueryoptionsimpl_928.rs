// Generated macro for impl_928 (impl)
macro_rules! Depcrate_features_gen_ClientQueryOptionsimpl_928 {
() => {
// Module: crate::features::gen_ClientQueryOptions
// Provides: {"impl_928"}
// Dependencies: {}
impl ClientQueryOptions { # [doc = "Construct a new `ClientQueryOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ClientQueryOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_include_uncontrolled()` instead."] pub fn include_uncontrolled (& mut self , val : bool) -> & mut Self { self . set_include_uncontrolled (val) ; self } # [cfg (feature = "ClientType")] # [deprecated = "Use `set_type()` instead."] pub fn type_ (& mut self , val : ClientType) -> & mut Self { self . set_type (val) ; self } }
};
}
