// Generated macro for impl_1605 (impl)
macro_rules! Depcrate_features_gen_DisplayNameResultimpl_1605 {
() => {
// Module: crate::features::gen_DisplayNameResult
// Provides: {"impl_1605"}
// Dependencies: {}
impl DisplayNameResult { # [doc = "Construct a new `DisplayNameResult`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `DisplayNameResult`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_locale()` instead."] pub fn locale (& mut self , val : & str) -> & mut Self { self . set_locale (val) ; self } # [deprecated = "Use `set_style()` instead."] pub fn style (& mut self , val : & str) -> & mut Self { self . set_style (val) ; self } # [deprecated = "Use `set_values()` instead."] pub fn values (& mut self , val : & :: js_sys :: Object) -> & mut Self { self . set_values (val) ; self } }
};
}
