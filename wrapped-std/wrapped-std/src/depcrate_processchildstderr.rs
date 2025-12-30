// Generated macro for ChildStderr (struct)
macro_rules! Depcrate_processChildStderr {
() => {
// Module: crate::process
// Provides: {"ChildStderr"}
// Dependencies: {}
# [doc = " A handle to a child process's stderr."] # [doc = ""] # [doc = " This struct is used in the [`stderr`] field on [`Child`]."] # [doc = ""] # [doc = " When an instance of `ChildStderr` is [dropped], the `ChildStderr`'s"] # [doc = " underlying file handle will be closed."] # [doc = ""] # [doc = " [`stderr`]: Child::stderr"] # [doc = " [dropped]: Drop"] # [stable (feature = "process" , since = "1.0.0")] pub struct ChildStderr { inner : AnonPipe , }
};
}
