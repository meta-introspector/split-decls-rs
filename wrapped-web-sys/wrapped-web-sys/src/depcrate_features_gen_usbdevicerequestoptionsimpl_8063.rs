// Generated macro for impl_8063 (impl)
macro_rules! Depcrate_features_gen_UsbDeviceRequestOptionsimpl_8063 {
() => {
// Module: crate::features::gen_UsbDeviceRequestOptions
// Provides: {"impl_8063"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl UsbDeviceRequestOptions { # [doc = "Construct a new `UsbDeviceRequestOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `UsbDeviceRequestOptions`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (filters : & :: wasm_bindgen :: JsValue) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_filters (filters) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_filters()` instead."] pub fn filters (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_filters (val) ; self } }
};
}
