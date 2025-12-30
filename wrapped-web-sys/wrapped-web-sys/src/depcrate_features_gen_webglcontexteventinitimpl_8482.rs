// Generated macro for impl_8482 (impl)
macro_rules! Depcrate_features_gen_WebGlContextEventInitimpl_8482 {
() => {
// Module: crate::features::gen_WebGlContextEventInit
// Provides: {"impl_8482"}
// Dependencies: {}
impl WebGlContextEventInit { # [doc = "Construct a new `WebGlContextEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `WebGlContextEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [deprecated = "Use `set_status_message()` instead."] pub fn status_message (& mut self , val : & str) -> & mut Self { self . set_status_message (val) ; self } }
};
}
