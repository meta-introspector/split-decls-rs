// Generated macro for impl_664 (impl)
macro_rules! Depcrate_features_gen_BluetoothDataFilterInitimpl_664 {
() => {
// Module: crate::features::gen_BluetoothDataFilterInit
// Provides: {"impl_664"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl BluetoothDataFilterInit { # [doc = "Construct a new `BluetoothDataFilterInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `BluetoothDataFilterInit`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_data_prefix()` instead."] pub fn data_prefix (& mut self , val : & :: js_sys :: Object) -> & mut Self { self . set_data_prefix (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_mask()` instead."] pub fn mask (& mut self , val : & :: js_sys :: Object) -> & mut Self { self . set_mask (val) ; self } }
};
}
