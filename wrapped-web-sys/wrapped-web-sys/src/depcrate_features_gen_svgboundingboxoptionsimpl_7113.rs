// Generated macro for impl_7113 (impl)
macro_rules! Depcrate_features_gen_SvgBoundingBoxOptionsimpl_7113 {
() => {
// Module: crate::features::gen_SvgBoundingBoxOptions
// Provides: {"impl_7113"}
// Dependencies: {}
impl SvgBoundingBoxOptions { # [doc = "Construct a new `SvgBoundingBoxOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_clipped()` instead."] pub fn clipped (& mut self , val : bool) -> & mut Self { self . set_clipped (val) ; self } # [deprecated = "Use `set_fill()` instead."] pub fn fill (& mut self , val : bool) -> & mut Self { self . set_fill (val) ; self } # [deprecated = "Use `set_markers()` instead."] pub fn markers (& mut self , val : bool) -> & mut Self { self . set_markers (val) ; self } # [deprecated = "Use `set_stroke()` instead."] pub fn stroke (& mut self , val : bool) -> & mut Self { self . set_stroke (val) ; self } }
};
}
