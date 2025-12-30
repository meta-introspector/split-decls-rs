// Generated macro for impl_1965 (impl)
macro_rules! Depcrate_features_gen_EventListenerimpl_1965 {
() => {
// Module: crate::features::gen_EventListener
// Provides: {"impl_1965"}
// Dependencies: {}
impl EventListener { # [doc = "Construct a new `EventListener`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `EventListener`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_handle_event()` instead."] pub fn handle_event (& mut self , val : & :: js_sys :: Function) -> & mut Self { self . set_handle_event (val) ; self } }
};
}
