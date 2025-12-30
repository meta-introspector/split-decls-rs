// Generated macro for impl_3778 (impl)
macro_rules! Depcrate_features_gen_HttpConnInfoimpl_3778 {
() => {
// Module: crate::features::gen_HttpConnInfo
// Provides: {"impl_3778"}
// Dependencies: {}
impl HttpConnInfo { # [doc = "Construct a new `HttpConnInfo`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `HttpConnInfo`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_protocol_version()` instead."] pub fn protocol_version (& mut self , val : & str) -> & mut Self { self . set_protocol_version (val) ; self } # [deprecated = "Use `set_rtt()` instead."] pub fn rtt (& mut self , val : u32) -> & mut Self { self . set_rtt (val) ; self } # [deprecated = "Use `set_ttl()` instead."] pub fn ttl (& mut self , val : u32) -> & mut Self { self . set_ttl (val) ; self } }
};
}
