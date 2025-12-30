// Generated macro for impl_3305 (impl)
macro_rules! Depcrate_features_gen_HidDeviceRequestOptionsimpl_3305 {
() => {
// Module: crate::features::gen_HidDeviceRequestOptions
// Provides: {"impl_3305"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl HidDeviceRequestOptions { # [doc = "Construct a new `HidDeviceRequestOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `HidDeviceRequestOptions`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (filters : & :: wasm_bindgen :: JsValue) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_filters (filters) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_filters()` instead."] pub fn filters (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_filters (val) ; self } }
};
}
