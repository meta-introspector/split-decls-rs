// Generated macro for impl_4315 (impl)
macro_rules! Depcrate_features_gen_MediaEncodingConfigurationimpl_4315 {
() => {
// Module: crate::features::gen_MediaEncodingConfiguration
// Provides: {"impl_4315"}
// Dependencies: {}
impl MediaEncodingConfiguration { # [cfg (feature = "MediaEncodingType")] # [doc = "Construct a new `MediaEncodingConfiguration`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `MediaEncodingConfiguration`, `MediaEncodingType`*"] pub fn new (type_ : MediaEncodingType) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_type (type_) ; ret } # [cfg (feature = "AudioConfiguration")] # [deprecated = "Use `set_audio()` instead."] pub fn audio (& mut self , val : & AudioConfiguration) -> & mut Self { self . set_audio (val) ; self } # [cfg (feature = "VideoConfiguration")] # [deprecated = "Use `set_video()` instead."] pub fn video (& mut self , val : & VideoConfiguration) -> & mut Self { self . set_video (val) ; self } # [cfg (feature = "MediaEncodingType")] # [deprecated = "Use `set_type()` instead."] pub fn type_ (& mut self , val : MediaEncodingType) -> & mut Self { self . set_type (val) ; self } }
};
}
