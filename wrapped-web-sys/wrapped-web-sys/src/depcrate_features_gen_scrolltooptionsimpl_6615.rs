// Generated macro for impl_6615 (impl)
macro_rules! Depcrate_features_gen_ScrollToOptionsimpl_6615 {
() => {
// Module: crate::features::gen_ScrollToOptions
// Provides: {"impl_6615"}
// Dependencies: {}
impl ScrollToOptions { # [doc = "Construct a new `ScrollToOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ScrollToOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (feature = "ScrollBehavior")] # [deprecated = "Use `set_behavior()` instead."] pub fn behavior (& mut self , val : ScrollBehavior) -> & mut Self { self . set_behavior (val) ; self } # [deprecated = "Use `set_left()` instead."] pub fn left (& mut self , val : f64) -> & mut Self { self . set_left (val) ; self } # [deprecated = "Use `set_top()` instead."] pub fn top (& mut self , val : f64) -> & mut Self { self . set_top (val) ; self } }
};
}
