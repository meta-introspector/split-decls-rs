// Generated macro for impl_6796 (impl)
macro_rules! Depcrate_features_gen_SocketsDictimpl_6796 {
() => {
// Module: crate::features::gen_SocketsDict
// Provides: {"impl_6796"}
// Dependencies: {}
impl SocketsDict { # [doc = "Construct a new `SocketsDict`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `SocketsDict`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_received()` instead."] pub fn received (& mut self , val : f64) -> & mut Self { self . set_received (val) ; self } # [deprecated = "Use `set_sent()` instead."] pub fn sent (& mut self , val : f64) -> & mut Self { self . set_sent (val) ; self } # [deprecated = "Use `set_sockets()` instead."] pub fn sockets (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_sockets (val) ; self } }
};
}
