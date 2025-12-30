// Generated macro for impl_3352 (impl)
macro_rules! Depcrate_features_gen_HitRegionOptionsimpl_3352 {
() => {
// Module: crate::features::gen_HitRegionOptions
// Provides: {"impl_3352"}
// Dependencies: {}
impl HitRegionOptions { # [doc = "Construct a new `HitRegionOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `HitRegionOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (feature = "Element")] # [deprecated = "Use `set_control()` instead."] pub fn control (& mut self , val : Option < & Element >) -> & mut Self { self . set_control (val) ; self } # [deprecated = "Use `set_id()` instead."] pub fn id (& mut self , val : & str) -> & mut Self { self . set_id (val) ; self } # [cfg (feature = "Path2d")] # [deprecated = "Use `set_path()` instead."] pub fn path (& mut self , val : Option < & Path2d >) -> & mut Self { self . set_path (val) ; self } }
};
}
