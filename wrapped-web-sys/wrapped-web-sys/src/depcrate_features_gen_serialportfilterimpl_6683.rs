// Generated macro for impl_6683 (impl)
macro_rules! Depcrate_features_gen_SerialPortFilterimpl_6683 {
() => {
// Module: crate::features::gen_SerialPortFilter
// Provides: {"impl_6683"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl SerialPortFilter { # [doc = "Construct a new `SerialPortFilter`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `SerialPortFilter`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_usb_product_id()` instead."] pub fn usb_product_id (& mut self , val : u16) -> & mut Self { self . set_usb_product_id (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_usb_vendor_id()` instead."] pub fn usb_vendor_id (& mut self , val : u16) -> & mut Self { self . set_usb_vendor_id (val) ; self } }
};
}
