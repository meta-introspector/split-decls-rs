// Generated macro for impl_3298 (impl)
macro_rules! Depcrate_features_gen_HidDeviceFilterimpl_3298 {
() => {
// Module: crate::features::gen_HidDeviceFilter
// Provides: {"impl_3298"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl HidDeviceFilter { # [doc = "Construct a new `HidDeviceFilter`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `HidDeviceFilter`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_product_id()` instead."] pub fn product_id (& mut self , val : u16) -> & mut Self { self . set_product_id (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_usage()` instead."] pub fn usage (& mut self , val : u16) -> & mut Self { self . set_usage (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_usage_page()` instead."] pub fn usage_page (& mut self , val : u16) -> & mut Self { self . set_usage_page (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_vendor_id()` instead."] pub fn vendor_id (& mut self , val : u32) -> & mut Self { self . set_vendor_id (val) ; self } }
};
}
