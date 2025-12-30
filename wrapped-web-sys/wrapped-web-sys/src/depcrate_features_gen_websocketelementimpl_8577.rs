// Generated macro for impl_8577 (impl)
macro_rules! Depcrate_features_gen_WebSocketElementimpl_8577 {
() => {
// Module: crate::features::gen_WebSocketElement
// Provides: {"impl_8577"}
// Dependencies: {}
impl WebSocketElement { # [doc = "Construct a new `WebSocketElement`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_encrypted()` instead."] pub fn encrypted (& mut self , val : bool) -> & mut Self { self . set_encrypted (val) ; self } # [deprecated = "Use `set_hostport()` instead."] pub fn hostport (& mut self , val : & str) -> & mut Self { self . set_hostport (val) ; self } # [deprecated = "Use `set_msgreceived()` instead."] pub fn msgreceived (& mut self , val : u32) -> & mut Self { self . set_msgreceived (val) ; self } # [deprecated = "Use `set_msgsent()` instead."] pub fn msgsent (& mut self , val : u32) -> & mut Self { self . set_msgsent (val) ; self } # [deprecated = "Use `set_receivedsize()` instead."] pub fn receivedsize (& mut self , val : f64) -> & mut Self { self . set_receivedsize (val) ; self } # [deprecated = "Use `set_sentsize()` instead."] pub fn sentsize (& mut self , val : f64) -> & mut Self { self . set_sentsize (val) ; self } }
};
}
