// Generated macro for impl_4959 (impl)
macro_rules! Depcrate_features_gen_ObserverCallbackimpl_4959 {
() => {
// Module: crate::features::gen_ObserverCallback
// Provides: {"impl_4959"}
// Dependencies: {}
impl ObserverCallback { # [doc = "Construct a new `ObserverCallback`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ObserverCallback`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_handle_event()` instead."] pub fn handle_event (& mut self , val : & :: js_sys :: Function) -> & mut Self { self . set_handle_event (val) ; self } }
};
}
