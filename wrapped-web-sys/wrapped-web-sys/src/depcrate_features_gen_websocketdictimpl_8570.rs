// Generated macro for impl_8570 (impl)
macro_rules! Depcrate_features_gen_WebSocketDictimpl_8570 {
() => {
// Module: crate::features::gen_WebSocketDict
// Provides: {"impl_8570"}
// Dependencies: {}
impl WebSocketDict { # [doc = "Construct a new `WebSocketDict`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `WebSocketDict`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_websockets()` instead."] pub fn websockets (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_websockets (val) ; self } }
};
}
