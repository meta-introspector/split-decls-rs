// Generated macro for impl_7656 (impl)
macro_rules! Depcrate_features_gen_TaskControllerInitimpl_7656 {
() => {
// Module: crate::features::gen_TaskControllerInit
// Provides: {"impl_7656"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl TaskControllerInit { # [doc = "Construct a new `TaskControllerInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `TaskControllerInit`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "TaskPriority")] # [deprecated = "Use `set_priority()` instead."] pub fn priority (& mut self , val : TaskPriority) -> & mut Self { self . set_priority (val) ; self } }
};
}
