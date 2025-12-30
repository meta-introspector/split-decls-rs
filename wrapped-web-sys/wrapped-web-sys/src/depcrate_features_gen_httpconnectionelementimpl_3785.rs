// Generated macro for impl_3785 (impl)
macro_rules! Depcrate_features_gen_HttpConnectionElementimpl_3785 {
() => {
// Module: crate::features::gen_HttpConnectionElement
// Provides: {"impl_3785"}
// Dependencies: {}
impl HttpConnectionElement { # [doc = "Construct a new `HttpConnectionElement`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_active()` instead."] pub fn active (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_active (val) ; self } # [deprecated = "Use `set_half_opens()` instead."] pub fn half_opens (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_half_opens (val) ; self } # [deprecated = "Use `set_host()` instead."] pub fn host (& mut self , val : & str) -> & mut Self { self . set_host (val) ; self } # [deprecated = "Use `set_idle()` instead."] pub fn idle (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_idle (val) ; self } # [deprecated = "Use `set_port()` instead."] pub fn port (& mut self , val : u32) -> & mut Self { self . set_port (val) ; self } # [deprecated = "Use `set_spdy()` instead."] pub fn spdy (& mut self , val : bool) -> & mut Self { self . set_spdy (val) ; self } # [deprecated = "Use `set_ssl()` instead."] pub fn ssl (& mut self , val : bool) -> & mut Self { self . set_ssl (val) ; self } }
};
}
