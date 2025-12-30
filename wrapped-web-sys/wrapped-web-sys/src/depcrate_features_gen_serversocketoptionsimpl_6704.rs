// Generated macro for impl_6704 (impl)
macro_rules! Depcrate_features_gen_ServerSocketOptionsimpl_6704 {
() => {
// Module: crate::features::gen_ServerSocketOptions
// Provides: {"impl_6704"}
// Dependencies: {}
impl ServerSocketOptions { # [doc = "Construct a new `ServerSocketOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ServerSocketOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (feature = "TcpSocketBinaryType")] # [deprecated = "Use `set_binary_type()` instead."] pub fn binary_type (& mut self , val : TcpSocketBinaryType) -> & mut Self { self . set_binary_type (val) ; self } }
};
}
