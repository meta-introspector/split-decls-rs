// Generated macro for impl_1750 (impl)
macro_rules! Depcrate_features_gen_DomRectInitimpl_1750 {
() => {
// Module: crate::features::gen_DomRectInit
// Provides: {"impl_1750"}
// Dependencies: {}
impl DomRectInit { # [doc = "Construct a new `DomRectInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `DomRectInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_height()` instead."] pub fn height (& mut self , val : f64) -> & mut Self { self . set_height (val) ; self } # [deprecated = "Use `set_width()` instead."] pub fn width (& mut self , val : f64) -> & mut Self { self . set_width (val) ; self } # [deprecated = "Use `set_x()` instead."] pub fn x (& mut self , val : f64) -> & mut Self { self . set_x (val) ; self } # [deprecated = "Use `set_y()` instead."] pub fn y (& mut self , val : f64) -> & mut Self { self . set_y (val) ; self } }
};
}
