// Generated macro for impl_253 (impl)
macro_rules! Depcrate_features_gen_AudioDecoderConfigimpl_253 {
() => {
// Module: crate::features::gen_AudioDecoderConfig
// Provides: {"impl_253"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl AudioDecoderConfig { # [doc = "Construct a new `AudioDecoderConfig`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `AudioDecoderConfig`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (codec : & str , number_of_channels : u32 , sample_rate : u32) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_codec (codec) ; ret . set_number_of_channels (number_of_channels) ; ret . set_sample_rate (sample_rate) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_codec()` instead."] pub fn codec (& mut self , val : & str) -> & mut Self { self . set_codec (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_description()` instead."] pub fn description (& mut self , val : & :: js_sys :: Object) -> & mut Self { self . set_description (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_number_of_channels()` instead."] pub fn number_of_channels (& mut self , val : u32) -> & mut Self { self . set_number_of_channels (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_sample_rate()` instead."] pub fn sample_rate (& mut self , val : u32) -> & mut Self { self . set_sample_rate (val) ; self } }
};
}
