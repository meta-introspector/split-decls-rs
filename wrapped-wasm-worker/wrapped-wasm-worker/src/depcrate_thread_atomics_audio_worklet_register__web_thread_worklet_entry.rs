// Generated macro for __web_thread_worklet_entry (function)
macro_rules! Depcrate_thread_atomics_audio_worklet_register__web_thread_worklet_entry {
() => {
// Module: crate::thread::atomics::audio_worklet::register
// Provides: {"__web_thread_worklet_entry"}
// Dependencies: {}
# [doc = " Entry function for the worklet."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `task` has to be a valid pointer to [`Task`]."] # [wasm_bindgen (skip_typescript)] # [allow (unreachable_pub)] # [cfg_attr (not (feature = "message") , allow (clippy :: needless_pass_by_value))] pub unsafe fn __web_thread_worklet_entry (task : NonNull < Task > , message : JsValue , # [cfg_attr (not (feature = "message") , allow (unused))] port : MessagePort ,) { # [cfg (feature = "message")] message :: MESSAGE_PORT . with (| cell | cell . set (port)) . expect ("found existing `MessagePort` in new thread") ; let task : Task = * unsafe { Box :: from_raw (task . as_ptr ()) } ; task (message) ; }
};
}
