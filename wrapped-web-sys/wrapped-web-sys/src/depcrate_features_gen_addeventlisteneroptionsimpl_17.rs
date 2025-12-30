// Generated macro for impl_17 (impl)
macro_rules! Depcrate_features_gen_AddEventListenerOptionsimpl_17 {
() => {
// Module: crate::features::gen_AddEventListenerOptions
// Provides: {"impl_17"}
// Dependencies: {}
impl AddEventListenerOptions { # [doc = "Construct a new `AddEventListenerOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `AddEventListenerOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_capture()` instead."] pub fn capture (& mut self , val : bool) -> & mut Self { self . set_capture (val) ; self } # [deprecated = "Use `set_once()` instead."] pub fn once (& mut self , val : bool) -> & mut Self { self . set_once (val) ; self } # [deprecated = "Use `set_passive()` instead."] pub fn passive (& mut self , val : bool) -> & mut Self { self . set_passive (val) ; self } # [cfg (feature = "AbortSignal")] # [deprecated = "Use `set_signal()` instead."] pub fn signal (& mut self , val : & AbortSignal) -> & mut Self { self . set_signal (val) ; self } }
};
}
