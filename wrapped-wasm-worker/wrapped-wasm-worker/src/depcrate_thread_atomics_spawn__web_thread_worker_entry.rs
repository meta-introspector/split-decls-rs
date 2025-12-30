// Generated macro for __web_thread_worker_entry (function)
macro_rules! Depcrate_thread_atomics_spawn__web_thread_worker_entry {
() => {
// Module: crate::thread::atomics::spawn
// Provides: {"__web_thread_worker_entry"}
// Dependencies: {}
# [doc = " Entry function for the worker."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `task` has to be a valid pointer to [`Task`]."] # [wasm_bindgen (skip_typescript)] # [allow (unreachable_pub)] pub async unsafe fn __web_thread_worker_entry (task : NonNull < TaskStatic > , message : JsValue) -> u32 { let task : Task < '_ > = * unsafe { Box :: from_raw (task . as_ptr ()) } ; task (message) . await }
};
}
