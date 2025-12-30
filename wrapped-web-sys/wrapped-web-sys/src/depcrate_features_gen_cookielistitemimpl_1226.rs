// Generated macro for impl_1226 (impl)
macro_rules! Depcrate_features_gen_CookieListItemimpl_1226 {
() => {
// Module: crate::features::gen_CookieListItem
// Provides: {"impl_1226"}
// Dependencies: {}
impl CookieListItem { # [doc = "Construct a new `CookieListItem`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `CookieListItem`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } # [deprecated = "Use `set_value()` instead."] pub fn value (& mut self , val : & str) -> & mut Self { self . set_value (val) ; self } }
};
}
