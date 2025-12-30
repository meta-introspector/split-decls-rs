// Generated macro for impl_4280 (impl)
macro_rules! Depcrate_features_gen_MediaDecodingConfigurationimpl_4280 {
() => {
// Module: crate::features::gen_MediaDecodingConfiguration
// Provides: {"impl_4280"}
// Dependencies: {}
impl MediaDecodingConfiguration { # [cfg (feature = "MediaDecodingType")] # [doc = "Construct a new `MediaDecodingConfiguration`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `MediaDecodingConfiguration`, `MediaDecodingType`*"] pub fn new (type_ : MediaDecodingType) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_type (type_) ; ret } # [cfg (feature = "AudioConfiguration")] # [deprecated = "Use `set_audio()` instead."] pub fn audio (& mut self , val : & AudioConfiguration) -> & mut Self { self . set_audio (val) ; self } # [cfg (feature = "VideoConfiguration")] # [deprecated = "Use `set_video()` instead."] pub fn video (& mut self , val : & VideoConfiguration) -> & mut Self { self . set_video (val) ; self } # [cfg (feature = "MediaDecodingType")] # [deprecated = "Use `set_type()` instead."] pub fn type_ (& mut self , val : MediaDecodingType) -> & mut Self { self . set_type (val) ; self } }
};
}
