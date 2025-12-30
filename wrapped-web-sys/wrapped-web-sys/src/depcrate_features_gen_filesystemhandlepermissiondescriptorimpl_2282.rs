// Generated macro for impl_2282 (impl)
macro_rules! Depcrate_features_gen_FileSystemHandlePermissionDescriptorimpl_2282 {
() => {
// Module: crate::features::gen_FileSystemHandlePermissionDescriptor
// Provides: {"impl_2282"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl FileSystemHandlePermissionDescriptor { # [doc = "Construct a new `FileSystemHandlePermissionDescriptor`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `FileSystemHandlePermissionDescriptor`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "FileSystemPermissionMode")] # [deprecated = "Use `set_mode()` instead."] pub fn mode (& mut self , val : FileSystemPermissionMode) -> & mut Self { self . set_mode (val) ; self } }
};
}
