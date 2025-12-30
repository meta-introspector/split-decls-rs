// Generated macro for impl_1791 (impl)
macro_rules! Depcrate_features_gen_DomWindowResizeEventDetailimpl_1791 {
() => {
// Module: crate::features::gen_DomWindowResizeEventDetail
// Provides: {"impl_1791"}
// Dependencies: {}
impl DomWindowResizeEventDetail { # [doc = "Construct a new `DomWindowResizeEventDetail`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `DomWindowResizeEventDetail`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_height()` instead."] pub fn height (& mut self , val : i32) -> & mut Self { self . set_height (val) ; self } # [deprecated = "Use `set_width()` instead."] pub fn width (& mut self , val : i32) -> & mut Self { self . set_width (val) ; self } }
};
}
