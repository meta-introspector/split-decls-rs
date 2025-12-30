// Generated macro for impl_985 (impl)
macro_rules! Depcrate_features_gen_ClipboardUnsanitizedFormatsimpl_985 {
() => {
// Module: crate::features::gen_ClipboardUnsanitizedFormats
// Provides: {"impl_985"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl ClipboardUnsanitizedFormats { # [doc = "Construct a new `ClipboardUnsanitizedFormats`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ClipboardUnsanitizedFormats`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_unsanitized()` instead."] pub fn unsanitized (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_unsanitized (val) ; self } }
};
}
