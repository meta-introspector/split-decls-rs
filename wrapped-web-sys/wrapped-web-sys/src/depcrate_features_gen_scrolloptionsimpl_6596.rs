// Generated macro for impl_6596 (impl)
macro_rules! Depcrate_features_gen_ScrollOptionsimpl_6596 {
() => {
// Module: crate::features::gen_ScrollOptions
// Provides: {"impl_6596"}
// Dependencies: {}
impl ScrollOptions { # [doc = "Construct a new `ScrollOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ScrollOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (feature = "ScrollBehavior")] # [deprecated = "Use `set_behavior()` instead."] pub fn behavior (& mut self , val : ScrollBehavior) -> & mut Self { self . set_behavior (val) ; self } }
};
}
