// Generated macro for impl_6778 (impl)
macro_rules! Depcrate_features_gen_SocketElementimpl_6778 {
() => {
// Module: crate::features::gen_SocketElement
// Provides: {"impl_6778"}
// Dependencies: {}
impl SocketElement { # [doc = "Construct a new `SocketElement`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `SocketElement`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_active()` instead."] pub fn active (& mut self , val : bool) -> & mut Self { self . set_active (val) ; self } # [deprecated = "Use `set_host()` instead."] pub fn host (& mut self , val : & str) -> & mut Self { self . set_host (val) ; self } # [deprecated = "Use `set_port()` instead."] pub fn port (& mut self , val : u32) -> & mut Self { self . set_port (val) ; self } # [deprecated = "Use `set_received()` instead."] pub fn received (& mut self , val : f64) -> & mut Self { self . set_received (val) ; self } # [deprecated = "Use `set_sent()` instead."] pub fn sent (& mut self , val : f64) -> & mut Self { self . set_sent (val) ; self } # [deprecated = "Use `set_tcp()` instead."] pub fn tcp (& mut self , val : bool) -> & mut Self { self . set_tcp (val) ; self } }
};
}
