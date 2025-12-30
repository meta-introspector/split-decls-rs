// Generated macro for impl_1972 (impl)
macro_rules! Depcrate_features_gen_EventListenerOptionsimpl_1972 {
() => {
// Module: crate::features::gen_EventListenerOptions
// Provides: {"impl_1972"}
// Dependencies: {}
impl EventListenerOptions { # [doc = "Construct a new `EventListenerOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `EventListenerOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_capture()` instead."] pub fn capture (& mut self , val : bool) -> & mut Self { self . set_capture (val) ; self } }
};
}
