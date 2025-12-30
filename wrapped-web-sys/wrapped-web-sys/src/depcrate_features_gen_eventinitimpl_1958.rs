// Generated macro for impl_1958 (impl)
macro_rules! Depcrate_features_gen_EventInitimpl_1958 {
() => {
// Module: crate::features::gen_EventInit
// Provides: {"impl_1958"}
// Dependencies: {}
impl EventInit { # [doc = "Construct a new `EventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `EventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } }
};
}
