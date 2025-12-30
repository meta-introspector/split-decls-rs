// Generated macro for impl_8278 (impl)
macro_rules! Depcrate_features_gen_VideoFrameCopyToOptionsimpl_8278 {
() => {
// Module: crate::features::gen_VideoFrameCopyToOptions
// Provides: {"impl_8278"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl VideoFrameCopyToOptions { # [doc = "Construct a new `VideoFrameCopyToOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `VideoFrameCopyToOptions`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_color_space()` instead."] pub fn color_space (& mut self , val : & str) -> & mut Self { self . set_color_space (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_format()` instead."] pub fn format (& mut self , val : & str) -> & mut Self { self . set_format (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_layout()` instead."] pub fn layout (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_layout (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "DomRectInit")] # [deprecated = "Use `set_rect()` instead."] pub fn rect (& mut self , val : & DomRectInit) -> & mut Self { self . set_rect (val) ; self } }
};
}
