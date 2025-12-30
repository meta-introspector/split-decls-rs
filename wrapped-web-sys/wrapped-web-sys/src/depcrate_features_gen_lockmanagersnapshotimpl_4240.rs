// Generated macro for impl_4240 (impl)
macro_rules! Depcrate_features_gen_LockManagerSnapshotimpl_4240 {
() => {
// Module: crate::features::gen_LockManagerSnapshot
// Provides: {"impl_4240"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl LockManagerSnapshot { # [doc = "Construct a new `LockManagerSnapshot`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `LockManagerSnapshot`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_held()` instead."] pub fn held (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_held (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_pending()` instead."] pub fn pending (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_pending (val) ; self } }
};
}
