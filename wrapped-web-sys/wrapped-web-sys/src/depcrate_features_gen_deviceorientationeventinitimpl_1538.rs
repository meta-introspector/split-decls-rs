// Generated macro for impl_1538 (impl)
macro_rules! Depcrate_features_gen_DeviceOrientationEventInitimpl_1538 {
() => {
// Module: crate::features::gen_DeviceOrientationEventInit
// Provides: {"impl_1538"}
// Dependencies: {}
impl DeviceOrientationEventInit { # [doc = "Construct a new `DeviceOrientationEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [deprecated = "Use `set_absolute()` instead."] pub fn absolute (& mut self , val : bool) -> & mut Self { self . set_absolute (val) ; self } # [deprecated = "Use `set_alpha()` instead."] pub fn alpha (& mut self , val : Option < f64 >) -> & mut Self { self . set_alpha (val) ; self } # [deprecated = "Use `set_beta()` instead."] pub fn beta (& mut self , val : Option < f64 >) -> & mut Self { self . set_beta (val) ; self } # [deprecated = "Use `set_gamma()` instead."] pub fn gamma (& mut self , val : Option < f64 >) -> & mut Self { self . set_gamma (val) ; self } }
};
}
