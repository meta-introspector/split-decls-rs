// Generated macro for activate_now (function)
macro_rules! Depcrateactivate_now {
() => {
// Module: crate
// Provides: {"activate_now"}
// Dependencies: {}
# [doc = " Triggers the killswitch, thereby scheduling all registered tasks to be"] # [doc = " killed."] # [doc = ""] # [doc = " Note: tasks are not killed synchronously in this function. This means"] # [doc = " `activate_now()` will return before all tasks have been stopped."] # [inline] pub fn activate_now () { TASK_KILLSWITCH . activate () ; }
};
}
