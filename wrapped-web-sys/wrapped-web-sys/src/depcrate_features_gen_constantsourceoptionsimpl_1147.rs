// Generated macro for impl_1147 (impl)
macro_rules! Depcrate_features_gen_ConstantSourceOptionsimpl_1147 {
() => {
// Module: crate::features::gen_ConstantSourceOptions
// Provides: {"impl_1147"}
// Dependencies: {}
impl ConstantSourceOptions { # [doc = "Construct a new `ConstantSourceOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ConstantSourceOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_offset()` instead."] pub fn offset (& mut self , val : f32) -> & mut Self { self . set_offset (val) ; self } }
};
}
