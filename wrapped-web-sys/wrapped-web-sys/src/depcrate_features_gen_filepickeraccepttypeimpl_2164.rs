// Generated macro for impl_2164 (impl)
macro_rules! Depcrate_features_gen_FilePickerAcceptTypeimpl_2164 {
() => {
// Module: crate::features::gen_FilePickerAcceptType
// Provides: {"impl_2164"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl FilePickerAcceptType { # [doc = "Construct a new `FilePickerAcceptType`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `FilePickerAcceptType`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_accept()` instead."] pub fn accept (& mut self , val : & :: js_sys :: Object) -> & mut Self { self . set_accept (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_description()` instead."] pub fn description (& mut self , val : & str) -> & mut Self { self . set_description (val) ; self } }
};
}
