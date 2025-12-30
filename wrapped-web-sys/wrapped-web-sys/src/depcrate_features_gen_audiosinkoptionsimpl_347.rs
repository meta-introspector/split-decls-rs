// Generated macro for impl_347 (impl)
macro_rules! Depcrate_features_gen_AudioSinkOptionsimpl_347 {
() => {
// Module: crate::features::gen_AudioSinkOptions
// Provides: {"impl_347"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl AudioSinkOptions { # [cfg (feature = "AudioSinkType")] # [doc = "Construct a new `AudioSinkOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `AudioSinkOptions`, `AudioSinkType`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (type_ : AudioSinkType) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_type (type_) ; ret } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "AudioSinkType")] # [deprecated = "Use `set_type()` instead."] pub fn type_ (& mut self , val : AudioSinkType) -> & mut Self { self . set_type (val) ; self } }
};
}
