// Generated macro for Yield (enum)
macro_rules! Depcrate_thread_poolYield {
() => {
// Module: crate::thread_pool
// Provides: {"Yield"}
// Dependencies: {}
# [doc = " Result of [`yield_now()`] or [`yield_local()`]."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Yield { # [doc = " Work was found and executed."] Executed , # [doc = " No available work was found."] Idle , }
};
}
