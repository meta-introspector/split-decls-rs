// Generated macro for impl_9077 (impl)
macro_rules! Depcrate_features_gen_XrSessionInitimpl_9077 {
() => {
// Module: crate::features::gen_XrSessionInit
// Provides: {"impl_9077"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl XrSessionInit { # [doc = "Construct a new `XrSessionInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `XrSessionInit`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_optional_features()` instead."] pub fn optional_features (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_optional_features (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_required_features()` instead."] pub fn required_features (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_required_features (val) ; self } }
};
}
