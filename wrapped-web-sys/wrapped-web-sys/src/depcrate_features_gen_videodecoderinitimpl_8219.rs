// Generated macro for impl_8219 (impl)
macro_rules! Depcrate_features_gen_VideoDecoderInitimpl_8219 {
() => {
// Module: crate::features::gen_VideoDecoderInit
// Provides: {"impl_8219"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl VideoDecoderInit { # [doc = "Construct a new `VideoDecoderInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `VideoDecoderInit`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (error : & :: js_sys :: Function , output : & :: js_sys :: Function) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_error (error) ; ret . set_output (output) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_error()` instead."] pub fn error (& mut self , val : & :: js_sys :: Function) -> & mut Self { self . set_error (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_output()` instead."] pub fn output (& mut self , val : & :: js_sys :: Function) -> & mut Self { self . set_output (val) ; self } }
};
}
