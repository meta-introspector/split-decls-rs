// Generated macro for other_360 (other)
macro_rules! Depcrate_thread_jsother_360 {
() => {
// Module: crate::thread::js
// Provides: {"other_360"}
// Dependencies: {}
# [cfg (not (web_sys_unstable_apis))] # [wasm_bindgen] extern "C" { # [doc = " [`Scheduler`](https://developer.mozilla.org/en-US/docs/Web/API/Scheduler) interface."] pub (super) type Scheduler ; # [doc = " Binding to [`Scheduler.postTask`](https://developer.mozilla.org/en-US/docs/Web/API/Scheduler/postTask)."] # [wasm_bindgen (method , js_name = postTask)] pub (super) fn post_task_with_options (this : & Scheduler , callback : & Function , options : & SchedulerPostTaskOptions ,) -> Promise ; # [doc = " Dictionary type of [`SchedulerPostTaskOptions`](https://developer.mozilla.org/en-US/docs/Web/API/Scheduler/postTask#options)."] pub (super) type SchedulerPostTaskOptions ; # [doc = " Setter for [`SchedulerPostTaskOptions.signal`](https://developer.mozilla.org/en-US/docs/Web/API/Scheduler/postTask#signal) property."] # [wasm_bindgen (method , setter , js_name = signal)] pub (super) fn set_signal (this : & SchedulerPostTaskOptions , signal : & AbortSignal) ; # [doc = " Setter for [`SchedulerPostTaskOptions.priority`](https://developer.mozilla.org/en-US/docs/Web/API/Scheduler/postTask#priority) property."] # [wasm_bindgen (method , setter , js_name = priority)] pub (super) fn set_priority (this : & SchedulerPostTaskOptions , priority : TaskPriority) ; }
};
}
