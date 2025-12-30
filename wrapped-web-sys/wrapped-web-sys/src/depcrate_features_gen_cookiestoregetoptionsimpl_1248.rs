// Generated macro for impl_1248 (impl)
macro_rules! Depcrate_features_gen_CookieStoreGetOptionsimpl_1248 {
() => {
// Module: crate::features::gen_CookieStoreGetOptions
// Provides: {"impl_1248"}
// Dependencies: {}
impl CookieStoreGetOptions { # [doc = "Construct a new `CookieStoreGetOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `CookieStoreGetOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } # [deprecated = "Use `set_url()` instead."] pub fn url (& mut self , val : & str) -> & mut Self { self . set_url (val) ; self } }
};
}
