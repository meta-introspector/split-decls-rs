// Generated macro for impl_7975 (impl)
macro_rules! Depcrate_features_gen_UdpOptionsimpl_7975 {
() => {
// Module: crate::features::gen_UdpOptions
// Provides: {"impl_7975"}
// Dependencies: {}
impl UdpOptions { # [doc = "Construct a new `UdpOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `UdpOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_address_reuse()` instead."] pub fn address_reuse (& mut self , val : bool) -> & mut Self { self . set_address_reuse (val) ; self } # [deprecated = "Use `set_local_address()` instead."] pub fn local_address (& mut self , val : & str) -> & mut Self { self . set_local_address (val) ; self } # [deprecated = "Use `set_local_port()` instead."] pub fn local_port (& mut self , val : u16) -> & mut Self { self . set_local_port (val) ; self } # [deprecated = "Use `set_loopback()` instead."] pub fn loopback (& mut self , val : bool) -> & mut Self { self . set_loopback (val) ; self } # [deprecated = "Use `set_remote_address()` instead."] pub fn remote_address (& mut self , val : & str) -> & mut Self { self . set_remote_address (val) ; self } # [deprecated = "Use `set_remote_port()` instead."] pub fn remote_port (& mut self , val : u16) -> & mut Self { self . set_remote_port (val) ; self } }
};
}
