// Generated macro for impl_3322 (impl)
macro_rules! Depcrate_features_gen_HidReportInfoimpl_3322 {
() => {
// Module: crate::features::gen_HidReportInfo
// Provides: {"impl_3322"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl HidReportInfo { # [doc = "Construct a new `HidReportInfo`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `HidReportInfo`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_items()` instead."] pub fn items (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_items (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_report_id()` instead."] pub fn report_id (& mut self , val : u8) -> & mut Self { self . set_report_id (val) ; self } }
};
}
