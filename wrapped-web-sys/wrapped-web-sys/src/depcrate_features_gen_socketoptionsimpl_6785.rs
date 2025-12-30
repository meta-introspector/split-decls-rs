// Generated macro for impl_6785 (impl)
macro_rules! Depcrate_features_gen_SocketOptionsimpl_6785 {
() => {
// Module: crate::features::gen_SocketOptions
// Provides: {"impl_6785"}
// Dependencies: {}
impl SocketOptions { # [doc = "Construct a new `SocketOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `SocketOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (feature = "TcpSocketBinaryType")] # [deprecated = "Use `set_binary_type()` instead."] pub fn binary_type (& mut self , val : TcpSocketBinaryType) -> & mut Self { self . set_binary_type (val) ; self } # [deprecated = "Use `set_use_secure_transport()` instead."] pub fn use_secure_transport (& mut self , val : bool) -> & mut Self { self . set_use_secure_transport (val) ; self } }
};
}
