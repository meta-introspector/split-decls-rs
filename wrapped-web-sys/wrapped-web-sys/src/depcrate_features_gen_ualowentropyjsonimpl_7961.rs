// Generated macro for impl_7961 (impl)
macro_rules! Depcrate_features_gen_UaLowEntropyJsonimpl_7961 {
() => {
// Module: crate::features::gen_UaLowEntropyJson
// Provides: {"impl_7961"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl UaLowEntropyJson { # [doc = "Construct a new `UaLowEntropyJson`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `UaLowEntropyJson`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_brands()` instead."] pub fn brands (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_brands (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_mobile()` instead."] pub fn mobile (& mut self , val : bool) -> & mut Self { self . set_mobile (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_platform()` instead."] pub fn platform (& mut self , val : & str) -> & mut Self { self . set_platform (val) ; self } }
};
}
