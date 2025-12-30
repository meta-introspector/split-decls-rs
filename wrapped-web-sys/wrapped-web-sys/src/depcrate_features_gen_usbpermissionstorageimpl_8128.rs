// Generated macro for impl_8128 (impl)
macro_rules! Depcrate_features_gen_UsbPermissionStorageimpl_8128 {
() => {
// Module: crate::features::gen_UsbPermissionStorage
// Provides: {"impl_8128"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl UsbPermissionStorage { # [doc = "Construct a new `UsbPermissionStorage`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `UsbPermissionStorage`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_allowed_devices()` instead."] pub fn allowed_devices (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_allowed_devices (val) ; self } }
};
}
