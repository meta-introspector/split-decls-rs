// Generated macro for Task (type)
macro_rules! Depcrate_thread_atomics_spawnTask {
() => {
// Module: crate::thread::atomics::spawn
// Provides: {"Task"}
// Dependencies: {}
# [doc = " Type of the task being sent to the worker."] type Task < 'scope > = Box < dyn 'scope + FnOnce (JsValue) -> Pin < Box < dyn 'scope + Future < Output = u32 > > > + Send > ;
};
}
