// Generated macro for impl_4273 (impl)
macro_rules! Depcrate_features_gen_MediaConfigurationimpl_4273 {
() => {
// Module: crate::features::gen_MediaConfiguration
// Provides: {"impl_4273"}
// Dependencies: {}
impl MediaConfiguration { # [doc = "Construct a new `MediaConfiguration`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `MediaConfiguration`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (feature = "AudioConfiguration")] # [deprecated = "Use `set_audio()` instead."] pub fn audio (& mut self , val : & AudioConfiguration) -> & mut Self { self . set_audio (val) ; self } # [cfg (feature = "VideoConfiguration")] # [deprecated = "Use `set_video()` instead."] pub fn video (& mut self , val : & VideoConfiguration) -> & mut Self { self . set_video (val) ; self } }
};
}
