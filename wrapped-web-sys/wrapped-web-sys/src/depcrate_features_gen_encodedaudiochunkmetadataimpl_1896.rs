// Generated macro for impl_1896 (impl)
macro_rules! Depcrate_features_gen_EncodedAudioChunkMetadataimpl_1896 {
() => {
// Module: crate::features::gen_EncodedAudioChunkMetadata
// Provides: {"impl_1896"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl EncodedAudioChunkMetadata { # [doc = "Construct a new `EncodedAudioChunkMetadata`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `EncodedAudioChunkMetadata`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "AudioDecoderConfig")] # [deprecated = "Use `set_decoder_config()` instead."] pub fn decoder_config (& mut self , val : & AudioDecoderConfig) -> & mut Self { self . set_decoder_config (val) ; self } }
};
}
