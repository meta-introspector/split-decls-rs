// Generated macro for impl_565 (impl)
macro_rules! Depcrate_features_gen_BasicCardRequestimpl_565 {
() => {
// Module: crate::features::gen_BasicCardRequest
// Provides: {"impl_565"}
// Dependencies: {}
impl BasicCardRequest { # [doc = "Construct a new `BasicCardRequest`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `BasicCardRequest`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_supported_networks()` instead."] pub fn supported_networks (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_supported_networks (val) ; self } # [deprecated = "Use `set_supported_types()` instead."] pub fn supported_types (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_supported_types (val) ; self } }
};
}
