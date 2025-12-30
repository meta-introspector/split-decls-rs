// Generated macro for ChildStdout (struct)
macro_rules! Depcrate_processChildStdout {
() => {
// Module: crate::process
// Provides: {"ChildStdout"}
// Dependencies: {}
# [doc = " A handle to a child process's standard output (stdout)."] # [doc = ""] # [doc = " This struct is used in the [`stdout`] field on [`Child`]."] # [doc = ""] # [doc = " When an instance of `ChildStdout` is [dropped], the `ChildStdout`'s"] # [doc = " underlying file handle will be closed."] # [doc = ""] # [doc = " [`stdout`]: Child::stdout"] # [doc = " [dropped]: Drop"] # [stable (feature = "process" , since = "1.0.0")] pub struct ChildStdout { inner : AnonPipe , }
};
}
