// Generated macro for impl_5539 (impl)
macro_rules! Depcrate_features_gen_ProgressEventInitimpl_5539 {
() => {
// Module: crate::features::gen_ProgressEventInit
// Provides: {"impl_5539"}
// Dependencies: {}
impl ProgressEventInit { # [doc = "Construct a new `ProgressEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [deprecated = "Use `set_length_computable()` instead."] pub fn length_computable (& mut self , val : bool) -> & mut Self { self . set_length_computable (val) ; self } # [deprecated = "Use `set_loaded()` instead."] pub fn loaded (& mut self , val : f64) -> & mut Self { self . set_loaded (val) ; self } # [deprecated = "Use `set_total()` instead."] pub fn total (& mut self , val : f64) -> & mut Self { self . set_total (val) ; self } }
};
}
