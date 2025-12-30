// Generated macro for impl_7725 (impl)
macro_rules! Depcrate_features_gen_TcpSocketErrorEventInitimpl_7725 {
() => {
// Module: crate::features::gen_TcpSocketErrorEventInit
// Provides: {"impl_7725"}
// Dependencies: {}
impl TcpSocketErrorEventInit { # [doc = "Construct a new `TcpSocketErrorEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `TcpSocketErrorEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [deprecated = "Use `set_message()` instead."] pub fn message (& mut self , val : & str) -> & mut Self { self . set_message (val) ; self } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } }
};
}
