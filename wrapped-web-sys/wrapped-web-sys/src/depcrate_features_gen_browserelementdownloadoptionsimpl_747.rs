// Generated macro for impl_747 (impl)
macro_rules! Depcrate_features_gen_BrowserElementDownloadOptionsimpl_747 {
() => {
// Module: crate::features::gen_BrowserElementDownloadOptions
// Provides: {"impl_747"}
// Dependencies: {}
impl BrowserElementDownloadOptions { # [doc = "Construct a new `BrowserElementDownloadOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `BrowserElementDownloadOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_filename()` instead."] pub fn filename (& mut self , val : Option < & str >) -> & mut Self { self . set_filename (val) ; self } # [deprecated = "Use `set_referrer()` instead."] pub fn referrer (& mut self , val : Option < & str >) -> & mut Self { self . set_referrer (val) ; self } }
};
}
