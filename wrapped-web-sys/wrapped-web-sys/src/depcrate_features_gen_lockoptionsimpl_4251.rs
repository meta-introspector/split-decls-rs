// Generated macro for impl_4251 (impl)
macro_rules! Depcrate_features_gen_LockOptionsimpl_4251 {
() => {
// Module: crate::features::gen_LockOptions
// Provides: {"impl_4251"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl LockOptions { # [doc = "Construct a new `LockOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `LockOptions`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_if_available()` instead."] pub fn if_available (& mut self , val : bool) -> & mut Self { self . set_if_available (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "LockMode")] # [deprecated = "Use `set_mode()` instead."] pub fn mode (& mut self , val : LockMode) -> & mut Self { self . set_mode (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "AbortSignal")] # [deprecated = "Use `set_signal()` instead."] pub fn signal (& mut self , val : & AbortSignal) -> & mut Self { self . set_signal (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_steal()` instead."] pub fn steal (& mut self , val : bool) -> & mut Self { self . set_steal (val) ; self } }
};
}
