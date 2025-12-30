// Generated macro for impl_8201 (impl)
macro_rules! Depcrate_features_gen_VideoConfigurationimpl_8201 {
() => {
// Module: crate::features::gen_VideoConfiguration
// Provides: {"impl_8201"}
// Dependencies: {}
impl VideoConfiguration { # [doc = "Construct a new `VideoConfiguration`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bitrate()` instead."] pub fn bitrate (& mut self , val : f64) -> & mut Self { self . set_bitrate (val) ; self } # [deprecated = "Use `set_content_type()` instead."] pub fn content_type (& mut self , val : & str) -> & mut Self { self . set_content_type (val) ; self } # [deprecated = "Use `set_framerate()` instead."] pub fn framerate (& mut self , val : & str) -> & mut Self { self . set_framerate (val) ; self } # [deprecated = "Use `set_height()` instead."] pub fn height (& mut self , val : u32) -> & mut Self { self . set_height (val) ; self } # [deprecated = "Use `set_width()` instead."] pub fn width (& mut self , val : u32) -> & mut Self { self . set_width (val) ; self } }
};
}
