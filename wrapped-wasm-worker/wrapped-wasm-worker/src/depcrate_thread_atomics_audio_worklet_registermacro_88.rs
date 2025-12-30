// Generated macro for macro_88 (macro)
macro_rules! Depcrate_thread_atomics_audio_worklet_registermacro_88 {
() => {
// Module: crate::thread::atomics::audio_worklet::register
// Provides: {"macro_88"}
// Dependencies: {}
thread_local ! { # [doc = " Cached [`JsValue`] holding index to worklet lock."] pub (in super :: super) static WORKLET_LOCK_INDEX : JsValue = super :: super :: i32_to_buffer_index (WORKLET_LOCK . as_ptr ()) . into () ; # [doc = " Cached [`Array`] holding indexes to worker and worklet locks."] pub (in super :: super) static THREAD_LOCK_INDEXES : Array = WORKLET_LOCK_INDEX . with (| worklet_index | { Array :: of2 (worklet_index , & super :: super :: i32_to_buffer_index (WORKER_LOCK . as_ptr ()) . into () ,) }) ; }
};
}
