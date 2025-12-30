// Generated macro for impl_5944 (impl)
macro_rules! Depcrate_features_gen_ResponseInitimpl_5944 {
() => {
// Module: crate::features::gen_ResponseInit
// Provides: {"impl_5944"}
// Dependencies: {}
impl ResponseInit { # [doc = "Construct a new `ResponseInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ResponseInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_headers()` instead."] pub fn headers (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_headers (val) ; self } # [deprecated = "Use `set_status()` instead."] pub fn status (& mut self , val : u16) -> & mut Self { self . set_status (val) ; self } # [deprecated = "Use `set_status_text()` instead."] pub fn status_text (& mut self , val : & str) -> & mut Self { self . set_status_text (val) ; self } }
};
}
