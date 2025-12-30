// Generated macro for ThreadStatus (enum)
macro_rules! Depcrate_unix_apple_processThreadStatus {
() => {
// Module: crate::unix::apple::process
// Provides: {"ThreadStatus"}
// Dependencies: {}
# [doc = " Enum describing the different status of a thread."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub (crate) enum ThreadStatus { # [doc = " Thread is running normally."] Running , # [doc = " Thread is stopped."] Stopped , # [doc = " Thread is waiting normally."] Waiting , # [doc = " Thread is in an uninterruptible wait"] Uninterruptible , # [doc = " Thread is halted at a clean point."] Halted , # [doc = " Unknown."] Unknown (i32) , }
};
}
