// Generated macro for impl_142 (impl)
macro_rules! Depcrate_features_gen_AnimationPropertyDetailsimpl_142 {
() => {
// Module: crate::features::gen_AnimationPropertyDetails
// Provides: {"impl_142"}
// Dependencies: {}
impl AnimationPropertyDetails { # [doc = "Construct a new `AnimationPropertyDetails`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `AnimationPropertyDetails`*"] pub fn new (property : & str , running_on_compositor : bool , values : & :: wasm_bindgen :: JsValue ,) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_property (property) ; ret . set_running_on_compositor (running_on_compositor) ; ret . set_values (values) ; ret } # [deprecated = "Use `set_property()` instead."] pub fn property (& mut self , val : & str) -> & mut Self { self . set_property (val) ; self } # [deprecated = "Use `set_running_on_compositor()` instead."] pub fn running_on_compositor (& mut self , val : bool) -> & mut Self { self . set_running_on_compositor (val) ; self } # [deprecated = "Use `set_values()` instead."] pub fn values (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_values (val) ; self } # [deprecated = "Use `set_warning()` instead."] pub fn warning (& mut self , val : & str) -> & mut Self { self . set_warning (val) ; self } }
};
}
