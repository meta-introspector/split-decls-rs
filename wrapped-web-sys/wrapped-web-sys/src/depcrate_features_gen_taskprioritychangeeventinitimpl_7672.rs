// Generated macro for impl_7672 (impl)
macro_rules! Depcrate_features_gen_TaskPriorityChangeEventInitimpl_7672 {
() => {
// Module: crate::features::gen_TaskPriorityChangeEventInit
// Provides: {"impl_7672"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl TaskPriorityChangeEventInit { # [cfg (feature = "TaskPriority")] # [doc = "Construct a new `TaskPriorityChangeEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `TaskPriority`, `TaskPriorityChangeEventInit`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (previous_priority : TaskPriority) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_previous_priority (previous_priority) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "TaskPriority")] # [deprecated = "Use `set_previous_priority()` instead."] pub fn previous_priority (& mut self , val : TaskPriority) -> & mut Self { self . set_previous_priority (val) ; self } }
};
}
