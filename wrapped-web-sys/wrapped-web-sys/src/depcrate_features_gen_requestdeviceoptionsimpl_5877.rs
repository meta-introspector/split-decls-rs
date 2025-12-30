// Generated macro for impl_5877 (impl)
macro_rules! Depcrate_features_gen_RequestDeviceOptionsimpl_5877 {
() => {
// Module: crate::features::gen_RequestDeviceOptions
// Provides: {"impl_5877"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl RequestDeviceOptions { # [doc = "Construct a new `RequestDeviceOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RequestDeviceOptions`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_accept_all_devices()` instead."] pub fn accept_all_devices (& mut self , val : bool) -> & mut Self { self . set_accept_all_devices (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_filters()` instead."] pub fn filters (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_filters (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_optional_services()` instead."] pub fn optional_services (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_optional_services (val) ; self } }
};
}
