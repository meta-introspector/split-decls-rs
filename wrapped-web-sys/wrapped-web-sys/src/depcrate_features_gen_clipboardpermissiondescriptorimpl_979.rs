// Generated macro for impl_979 (impl)
macro_rules! Depcrate_features_gen_ClipboardPermissionDescriptorimpl_979 {
() => {
// Module: crate::features::gen_ClipboardPermissionDescriptor
// Provides: {"impl_979"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl ClipboardPermissionDescriptor { # [cfg (feature = "PermissionName")] # [doc = "Construct a new `ClipboardPermissionDescriptor`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ClipboardPermissionDescriptor`, `PermissionName`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (name : PermissionName) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_name (name) ; ret } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "PermissionName")] # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : PermissionName) -> & mut Self { self . set_name (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_allow_without_gesture()` instead."] pub fn allow_without_gesture (& mut self , val : bool) -> & mut Self { self . set_allow_without_gesture (val) ; self } }
};
}
