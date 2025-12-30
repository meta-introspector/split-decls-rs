// Generated macro for impl_8117 (impl)
macro_rules! Depcrate_features_gen_UsbPermissionDescriptorimpl_8117 {
() => {
// Module: crate::features::gen_UsbPermissionDescriptor
// Provides: {"impl_8117"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl UsbPermissionDescriptor { # [cfg (feature = "PermissionName")] # [doc = "Construct a new `UsbPermissionDescriptor`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `PermissionName`, `UsbPermissionDescriptor`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (name : PermissionName) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_name (name) ; ret } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "PermissionName")] # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : PermissionName) -> & mut Self { self . set_name (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_filters()` instead."] pub fn filters (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_filters (val) ; self } }
};
}
