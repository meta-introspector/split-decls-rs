// Generated macro for impl_5296 (impl)
macro_rules! Depcrate_features_gen_PeriodicWaveOptionsimpl_5296 {
() => {
// Module: crate::features::gen_PeriodicWaveOptions
// Provides: {"impl_5296"}
// Dependencies: {}
impl PeriodicWaveOptions { # [doc = "Construct a new `PeriodicWaveOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `PeriodicWaveOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_disable_normalization()` instead."] pub fn disable_normalization (& mut self , val : bool) -> & mut Self { self . set_disable_normalization (val) ; self } # [deprecated = "Use `set_imag()` instead."] pub fn imag (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_imag (val) ; self } # [deprecated = "Use `set_real()` instead."] pub fn real (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_real (val) ; self } }
};
}
