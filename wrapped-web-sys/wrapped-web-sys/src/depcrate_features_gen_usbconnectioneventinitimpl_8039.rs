// Generated macro for impl_8039 (impl)
macro_rules! Depcrate_features_gen_UsbConnectionEventInitimpl_8039 {
() => {
// Module: crate::features::gen_UsbConnectionEventInit
// Provides: {"impl_8039"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl UsbConnectionEventInit { # [cfg (feature = "UsbDevice")] # [doc = "Construct a new `UsbConnectionEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `UsbConnectionEventInit`, `UsbDevice`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (device : & UsbDevice) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_device (device) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "UsbDevice")] # [deprecated = "Use `set_device()` instead."] pub fn device (& mut self , val : & UsbDevice) -> & mut Self { self . set_device (val) ; self } }
};
}
