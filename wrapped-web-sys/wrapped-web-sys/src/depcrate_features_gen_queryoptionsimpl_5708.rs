// Generated macro for impl_5708 (impl)
macro_rules! Depcrate_features_gen_QueryOptionsimpl_5708 {
() => {
// Module: crate::features::gen_QueryOptions
// Provides: {"impl_5708"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl QueryOptions { # [doc = "Construct a new `QueryOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `QueryOptions`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_postscript_names()` instead."] pub fn postscript_names (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_postscript_names (val) ; self } }
};
}
