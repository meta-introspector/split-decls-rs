// Generated macro for impl_70 (impl)
macro_rules! Depcrate_features_gen_AllowedBluetoothDeviceimpl_70 {
() => {
// Module: crate::features::gen_AllowedBluetoothDevice
// Provides: {"impl_70"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl AllowedBluetoothDevice { # [doc = "Construct a new `AllowedBluetoothDevice`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `AllowedBluetoothDevice`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (allowed_services : & :: wasm_bindgen :: JsValue , device_id : & str , may_use_gatt : bool ,) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_allowed_services (allowed_services) ; ret . set_device_id (device_id) ; ret . set_may_use_gatt (may_use_gatt) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_allowed_services()` instead."] pub fn allowed_services (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_allowed_services (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_device_id()` instead."] pub fn device_id (& mut self , val : & str) -> & mut Self { self . set_device_id (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_may_use_gatt()` instead."] pub fn may_use_gatt (& mut self , val : bool) -> & mut Self { self . set_may_use_gatt (val) ; self } }
};
}
