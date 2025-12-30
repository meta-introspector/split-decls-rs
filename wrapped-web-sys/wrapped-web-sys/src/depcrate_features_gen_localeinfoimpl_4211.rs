// Generated macro for impl_4211 (impl)
macro_rules! Depcrate_features_gen_LocaleInfoimpl_4211 {
() => {
// Module: crate::features::gen_LocaleInfo
// Provides: {"impl_4211"}
// Dependencies: {}
impl LocaleInfo { # [doc = "Construct a new `LocaleInfo`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `LocaleInfo`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_direction()` instead."] pub fn direction (& mut self , val : & str) -> & mut Self { self . set_direction (val) ; self } # [deprecated = "Use `set_locale()` instead."] pub fn locale (& mut self , val : & str) -> & mut Self { self . set_locale (val) ; self } }
};
}
