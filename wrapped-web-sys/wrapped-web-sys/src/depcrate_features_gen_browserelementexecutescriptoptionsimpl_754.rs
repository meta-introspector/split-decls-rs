// Generated macro for impl_754 (impl)
macro_rules! Depcrate_features_gen_BrowserElementExecuteScriptOptionsimpl_754 {
() => {
// Module: crate::features::gen_BrowserElementExecuteScriptOptions
// Provides: {"impl_754"}
// Dependencies: {}
impl BrowserElementExecuteScriptOptions { # [doc = "Construct a new `BrowserElementExecuteScriptOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `BrowserElementExecuteScriptOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_origin()` instead."] pub fn origin (& mut self , val : Option < & str >) -> & mut Self { self . set_origin (val) ; self } # [deprecated = "Use `set_url()` instead."] pub fn url (& mut self , val : Option < & str >) -> & mut Self { self . set_url (val) ; self } }
};
}
