// Generated macro for impl_6535 (impl)
macro_rules! Depcrate_features_gen_SchedulerPostTaskOptionsimpl_6535 {
() => {
// Module: crate::features::gen_SchedulerPostTaskOptions
// Provides: {"impl_6535"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl SchedulerPostTaskOptions { # [doc = "Construct a new `SchedulerPostTaskOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `SchedulerPostTaskOptions`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_delay()` instead."] pub fn delay (& mut self , val : f64) -> & mut Self { self . set_delay (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "TaskPriority")] # [deprecated = "Use `set_priority()` instead."] pub fn priority (& mut self , val : TaskPriority) -> & mut Self { self . set_priority (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "AbortSignal")] # [deprecated = "Use `set_signal()` instead."] pub fn signal (& mut self , val : & AbortSignal) -> & mut Self { self . set_signal (val) ; self } }
};
}
