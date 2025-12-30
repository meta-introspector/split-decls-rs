// Generated macro for impl_1871 (impl)
macro_rules! Depcrate_features_gen_ElementCreationOptionsimpl_1871 {
() => {
// Module: crate::features::gen_ElementCreationOptions
// Provides: {"impl_1871"}
// Dependencies: {}
impl ElementCreationOptions { # [doc = "Construct a new `ElementCreationOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ElementCreationOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_is()` instead."] pub fn is (& mut self , val : & str) -> & mut Self { self . set_is (val) ; self } # [deprecated = "Use `set_pseudo()` instead."] pub fn pseudo (& mut self , val : & str) -> & mut Self { self . set_pseudo (val) ; self } }
};
}
