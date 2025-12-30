// Generated macro for ChildStdin (struct)
macro_rules! Depcrate_processChildStdin {
() => {
// Module: crate::process
// Provides: {"ChildStdin"}
// Dependencies: {}
# [doc = " A handle to a child process's standard input (stdin)."] # [doc = ""] # [doc = " This struct is used in the [`stdin`] field on [`Child`]."] # [doc = ""] # [doc = " When an instance of `ChildStdin` is [dropped], the `ChildStdin`'s underlying"] # [doc = " file handle will be closed. If the child process was blocked on input prior"] # [doc = " to being dropped, it will become unblocked after dropping."] # [doc = ""] # [doc = " [`stdin`]: Child::stdin"] # [doc = " [dropped]: Drop"] # [stable (feature = "process" , since = "1.0.0")] pub struct ChildStdin { inner : AnonPipe , }
};
}
