// Generated macro for impl_4336 (impl)
macro_rules! Depcrate_features_gen_MediaImageimpl_4336 {
() => {
// Module: crate::features::gen_MediaImage
// Provides: {"impl_4336"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl MediaImage { # [doc = "Construct a new `MediaImage`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `MediaImage`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (src : & str) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_src (src) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_sizes()` instead."] pub fn sizes (& mut self , val : & str) -> & mut Self { self . set_sizes (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_src()` instead."] pub fn src (& mut self , val : & str) -> & mut Self { self . set_src (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_type()` instead."] pub fn type_ (& mut self , val : & str) -> & mut Self { self . set_type (val) ; self } }
};
}
