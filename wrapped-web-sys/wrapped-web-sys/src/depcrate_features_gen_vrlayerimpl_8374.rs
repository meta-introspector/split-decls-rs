// Generated macro for impl_8374 (impl)
macro_rules! Depcrate_features_gen_VrLayerimpl_8374 {
() => {
// Module: crate::features::gen_VrLayer
// Provides: {"impl_8374"}
// Dependencies: {}
impl VrLayer { # [doc = "Construct a new `VrLayer`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `VrLayer`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_left_bounds()` instead."] pub fn left_bounds (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_left_bounds (val) ; self } # [deprecated = "Use `set_right_bounds()` instead."] pub fn right_bounds (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_right_bounds (val) ; self } # [cfg (feature = "HtmlCanvasElement")] # [deprecated = "Use `set_source()` instead."] pub fn source (& mut self , val : Option < & HtmlCanvasElement >) -> & mut Self { self . set_source (val) ; self } }
};
}
