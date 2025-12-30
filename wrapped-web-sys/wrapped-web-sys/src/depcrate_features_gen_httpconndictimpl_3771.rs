// Generated macro for impl_3771 (impl)
macro_rules! Depcrate_features_gen_HttpConnDictimpl_3771 {
() => {
// Module: crate::features::gen_HttpConnDict
// Provides: {"impl_3771"}
// Dependencies: {}
impl HttpConnDict { # [doc = "Construct a new `HttpConnDict`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `HttpConnDict`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_connections()` instead."] pub fn connections (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_connections (val) ; self } }
};
}
