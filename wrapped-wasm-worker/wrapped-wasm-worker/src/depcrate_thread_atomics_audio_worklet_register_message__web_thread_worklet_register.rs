// Generated macro for __web_thread_worklet_register (function)
macro_rules! Depcrate_thread_atomics_audio_worklet_register_message__web_thread_worklet_register {
() => {
// Module: crate::thread::atomics::audio_worklet::register::message
// Provides: {"__web_thread_worklet_register"}
// Dependencies: {}
# [doc = " Register function for the worklet."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `data` has to be a valid pointer to [`Data`]."] # [wasm_bindgen (skip_typescript)] # [allow (private_interfaces , unreachable_pub)] pub unsafe fn __web_thread_worklet_register (data : NonNull < Data >) { let data : Data = * unsafe { Box :: from_raw (data . as_ptr ()) } ; Thread :: register (data . thread) ; data . memory_sender . send (ThreadMemory :: new (data . stack_size)) ; }
};
}
