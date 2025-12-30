// Generated macro for impl_4228 (impl)
macro_rules! Depcrate_features_gen_LockInfoimpl_4228 {
() => {
// Module: crate::features::gen_LockInfo
// Provides: {"impl_4228"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl LockInfo { # [doc = "Construct a new `LockInfo`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `LockInfo`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_client_id()` instead."] pub fn client_id (& mut self , val : & str) -> & mut Self { self . set_client_id (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "LockMode")] # [deprecated = "Use `set_mode()` instead."] pub fn mode (& mut self , val : LockMode) -> & mut Self { self . set_mode (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } }
};
}
