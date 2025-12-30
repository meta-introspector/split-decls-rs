// Generated macro for impl_1514 (impl)
macro_rules! Depcrate_features_gen_DeviceLightEventInitimpl_1514 {
() => {
// Module: crate::features::gen_DeviceLightEventInit
// Provides: {"impl_1514"}
// Dependencies: {}
impl DeviceLightEventInit { # [doc = "Construct a new `DeviceLightEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `DeviceLightEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [deprecated = "Use `set_value()` instead."] pub fn value (& mut self , val : f64) -> & mut Self { self . set_value (val) ; self } }
};
}
