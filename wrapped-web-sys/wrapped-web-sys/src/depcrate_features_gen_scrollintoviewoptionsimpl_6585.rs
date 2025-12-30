// Generated macro for impl_6585 (impl)
macro_rules! Depcrate_features_gen_ScrollIntoViewOptionsimpl_6585 {
() => {
// Module: crate::features::gen_ScrollIntoViewOptions
// Provides: {"impl_6585"}
// Dependencies: {}
impl ScrollIntoViewOptions { # [doc = "Construct a new `ScrollIntoViewOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ScrollIntoViewOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (feature = "ScrollBehavior")] # [deprecated = "Use `set_behavior()` instead."] pub fn behavior (& mut self , val : ScrollBehavior) -> & mut Self { self . set_behavior (val) ; self } # [cfg (feature = "ScrollLogicalPosition")] # [deprecated = "Use `set_block()` instead."] pub fn block (& mut self , val : ScrollLogicalPosition) -> & mut Self { self . set_block (val) ; self } # [cfg (feature = "ScrollLogicalPosition")] # [deprecated = "Use `set_inline()` instead."] pub fn inline (& mut self , val : ScrollLogicalPosition) -> & mut Self { self . set_inline (val) ; self } }
};
}
