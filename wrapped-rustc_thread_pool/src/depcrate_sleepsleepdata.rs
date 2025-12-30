// Generated macro for SleepData (struct)
macro_rules! Depcrate_sleepSleepData {
() => {
// Module: crate::sleep
// Provides: {"SleepData"}
// Dependencies: {}
struct SleepData { # [doc = " The number of threads in the thread pool."] worker_count : usize , # [doc = " The number of threads in the thread pool which are running and"] # [doc = " aren't blocked in user code or sleeping."] active_threads : usize , # [doc = " The number of threads which are blocked in user code."] # [doc = " This doesn't include threads blocked by this module."] blocked_threads : usize , }
};
}
