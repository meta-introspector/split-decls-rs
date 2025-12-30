// Generated macro for impl_911 (impl)
macro_rules! Depcrate_features_gen_ChromeFilePropertyBagimpl_911 {
() => {
// Module: crate::features::gen_ChromeFilePropertyBag
// Provides: {"impl_911"}
// Dependencies: {}
impl ChromeFilePropertyBag { # [doc = "Construct a new `ChromeFilePropertyBag`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_last_modified()` instead."] pub fn last_modified (& mut self , val : f64) -> & mut Self { self . set_last_modified (val) ; self } # [deprecated = "Use `set_type()` instead."] pub fn type_ (& mut self , val : & str) -> & mut Self { self . set_type (val) ; self } # [deprecated = "Use `set_existence_check()` instead."] pub fn existence_check (& mut self , val : bool) -> & mut Self { self . set_existence_check (val) ; self } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } }
};
}
