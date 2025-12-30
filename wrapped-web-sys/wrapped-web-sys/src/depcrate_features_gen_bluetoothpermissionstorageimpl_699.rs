// Generated macro for impl_699 (impl)
macro_rules! Depcrate_features_gen_BluetoothPermissionStorageimpl_699 {
() => {
// Module: crate::features::gen_BluetoothPermissionStorage
// Provides: {"impl_699"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl BluetoothPermissionStorage { # [doc = "Construct a new `BluetoothPermissionStorage`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `BluetoothPermissionStorage`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (allowed_devices : & :: wasm_bindgen :: JsValue) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_allowed_devices (allowed_devices) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_allowed_devices()` instead."] pub fn allowed_devices (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_allowed_devices (val) ; self } }
};
}
