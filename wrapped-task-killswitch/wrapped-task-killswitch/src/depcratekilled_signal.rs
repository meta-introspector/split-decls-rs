// Generated macro for killed_signal (function)
macro_rules! Depcratekilled_signal {
() => {
// Module: crate
// Provides: {"killed_signal"}
// Dependencies: {}
# [doc = " Returns a future that resolves when all registered tasks have been killed,"] # [doc = " after [`activate_now`] has been called."] # [doc = ""] # [doc = " Note: tokio does not kill a task until the next time it yields to the"] # [doc = " runtime. This means some killed tasks may still be running by the time this"] # [doc = " Future resolves."] # [inline] pub fn killed_signal () -> impl Future < Output = () > + Send + 'static { TASK_KILLSWITCH . killed () }
};
}
