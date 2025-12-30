// Generated macro for impl_5421 (impl)
macro_rules! Depcrate_features_gen_PositionOptionsimpl_5421 {
() => {
// Module: crate::features::gen_PositionOptions
// Provides: {"impl_5421"}
// Dependencies: {}
impl PositionOptions { # [doc = "Construct a new `PositionOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `PositionOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_enable_high_accuracy()` instead."] pub fn enable_high_accuracy (& mut self , val : bool) -> & mut Self { self . set_enable_high_accuracy (val) ; self } # [deprecated = "Use `set_maximum_age()` instead."] pub fn maximum_age (& mut self , val : u32) -> & mut Self { self . set_maximum_age (val) ; self } # [deprecated = "Use `set_timeout()` instead."] pub fn timeout (& mut self , val : u32) -> & mut Self { self . set_timeout (val) ; self } }
};
}
