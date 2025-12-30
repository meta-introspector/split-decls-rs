// Generated macro for Task (type)
macro_rules! Depcrate_thread_atomics_audio_worklet_registerTask {
() => {
// Module: crate::thread::atomics::audio_worklet::register
// Provides: {"Task"}
// Dependencies: {}
# [doc = " Type of the task being sent to the worklet."] type Task = Box < dyn 'static + FnOnce (JsValue) + Send > ;
};
}
