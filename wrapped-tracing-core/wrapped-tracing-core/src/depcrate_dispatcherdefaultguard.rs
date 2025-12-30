// Generated macro for DefaultGuard (struct)
macro_rules! Depcrate_dispatcherDefaultGuard {
() => {
// Module: crate::dispatcher
// Provides: {"DefaultGuard"}
// Dependencies: {}
# [doc = " A guard that resets the current default dispatcher to the prior"] # [doc = " default dispatcher when dropped."] # [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] # [derive (Debug)] pub struct DefaultGuard (Option < Dispatch >) ;
};
}
