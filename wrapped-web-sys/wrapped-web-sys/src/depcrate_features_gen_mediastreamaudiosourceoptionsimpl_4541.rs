// Generated macro for impl_4541 (impl)
macro_rules! Depcrate_features_gen_MediaStreamAudioSourceOptionsimpl_4541 {
() => {
// Module: crate::features::gen_MediaStreamAudioSourceOptions
// Provides: {"impl_4541"}
// Dependencies: {}
impl MediaStreamAudioSourceOptions { # [cfg (feature = "MediaStream")] # [doc = "Construct a new `MediaStreamAudioSourceOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamAudioSourceOptions`*"] pub fn new (media_stream : & MediaStream) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_media_stream (media_stream) ; ret } # [cfg (feature = "MediaStream")] # [deprecated = "Use `set_media_stream()` instead."] pub fn media_stream (& mut self , val : & MediaStream) -> & mut Self { self . set_media_stream (val) ; self } }
};
}
