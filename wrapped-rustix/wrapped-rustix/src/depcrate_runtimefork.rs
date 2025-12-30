// Generated macro for Fork (enum)
macro_rules! Depcrate_runtimeFork {
() => {
// Module: crate::runtime
// Provides: {"Fork"}
// Dependencies: {}
# [doc = " Regular Unix `fork` doesn't tell the child its own PID because it assumes"] # [doc = " the child can just do `getpid`. That's true, but it's more fun if it"] # [doc = " doesn't have to."] pub enum Fork { # [doc = " This is returned in the child process after a `fork`. It holds the PID"] # [doc = " of the child."] Child (Pid) , # [doc = " This is returned in the parent process after a `fork`. It holds the PID"] # [doc = " of the child."] ParentOf (Pid) , }
};
}
