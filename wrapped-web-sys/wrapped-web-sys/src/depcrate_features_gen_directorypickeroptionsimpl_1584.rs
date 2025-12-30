// Generated macro for impl_1584 (impl)
macro_rules! Depcrate_features_gen_DirectoryPickerOptionsimpl_1584 {
() => {
// Module: crate::features::gen_DirectoryPickerOptions
// Provides: {"impl_1584"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl DirectoryPickerOptions { # [doc = "Construct a new `DirectoryPickerOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `DirectoryPickerOptions`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_id()` instead."] pub fn id (& mut self , val : & str) -> & mut Self { self . set_id (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "FileSystemPermissionMode")] # [deprecated = "Use `set_mode()` instead."] pub fn mode (& mut self , val : FileSystemPermissionMode) -> & mut Self { self . set_mode (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_start_in()` instead."] pub fn start_in (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_start_in (val) ; self } }
};
}
