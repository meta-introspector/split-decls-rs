// Generated macro for impl_2171 (impl)
macro_rules! Depcrate_features_gen_FilePickerOptionsimpl_2171 {
() => {
// Module: crate::features::gen_FilePickerOptions
// Provides: {"impl_2171"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl FilePickerOptions { # [doc = "Construct a new `FilePickerOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `FilePickerOptions`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_exclude_accept_all_option()` instead."] pub fn exclude_accept_all_option (& mut self , val : bool) -> & mut Self { self . set_exclude_accept_all_option (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_id()` instead."] pub fn id (& mut self , val : & str) -> & mut Self { self . set_id (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_start_in()` instead."] pub fn start_in (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_start_in (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_types()` instead."] pub fn types (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_types (val) ; self } }
};
}
