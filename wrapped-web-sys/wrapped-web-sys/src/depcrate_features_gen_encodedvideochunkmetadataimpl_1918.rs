// Generated macro for impl_1918 (impl)
macro_rules! Depcrate_features_gen_EncodedVideoChunkMetadataimpl_1918 {
() => {
// Module: crate::features::gen_EncodedVideoChunkMetadata
// Provides: {"impl_1918"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl EncodedVideoChunkMetadata { # [doc = "Construct a new `EncodedVideoChunkMetadata`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkMetadata`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_alpha_side_data()` instead."] pub fn alpha_side_data (& mut self , val : & :: js_sys :: Object) -> & mut Self { self . set_alpha_side_data (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "VideoDecoderConfig")] # [deprecated = "Use `set_decoder_config()` instead."] pub fn decoder_config (& mut self , val : & VideoDecoderConfig) -> & mut Self { self . set_decoder_config (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "SvcOutputMetadata")] # [deprecated = "Use `set_svc()` instead."] pub fn svc (& mut self , val : & SvcOutputMetadata) -> & mut Self { self . set_svc (val) ; self } }
};
}
