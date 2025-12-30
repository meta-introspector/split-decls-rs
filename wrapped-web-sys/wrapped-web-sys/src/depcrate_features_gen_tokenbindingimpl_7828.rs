// Generated macro for impl_7828 (impl)
macro_rules! Depcrate_features_gen_TokenBindingimpl_7828 {
() => {
// Module: crate::features::gen_TokenBinding
// Provides: {"impl_7828"}
// Dependencies: {}
impl TokenBinding { # [doc = "Construct a new `TokenBinding`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `TokenBinding`*"] pub fn new (status : & str) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_status (status) ; ret } # [deprecated = "Use `set_id()` instead."] pub fn id (& mut self , val : & str) -> & mut Self { self . set_id (val) ; self } # [deprecated = "Use `set_status()` instead."] pub fn status (& mut self , val : & str) -> & mut Self { self . set_status (val) ; self } }
};
}
