// Generated macro for impl_2178 (impl)
macro_rules! Depcrate_features_gen_FilePropertyBagimpl_2178 {
() => {
// Module: crate::features::gen_FilePropertyBag
// Provides: {"impl_2178"}
// Dependencies: {}
impl FilePropertyBag { # [doc = "Construct a new `FilePropertyBag`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `FilePropertyBag`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_last_modified()` instead."] pub fn last_modified (& mut self , val : f64) -> & mut Self { self . set_last_modified (val) ; self } # [deprecated = "Use `set_type()` instead."] pub fn type_ (& mut self , val : & str) -> & mut Self { self . set_type (val) ; self } }
};
}
