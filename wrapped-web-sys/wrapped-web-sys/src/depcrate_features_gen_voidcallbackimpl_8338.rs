// Generated macro for impl_8338 (impl)
macro_rules! Depcrate_features_gen_VoidCallbackimpl_8338 {
() => {
// Module: crate::features::gen_VoidCallback
// Provides: {"impl_8338"}
// Dependencies: {}
impl VoidCallback { # [doc = "Construct a new `VoidCallback`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `VoidCallback`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_handle_event()` instead."] pub fn handle_event (& mut self , val : & :: js_sys :: Function) -> & mut Self { self . set_handle_event (val) ; self } }
};
}
