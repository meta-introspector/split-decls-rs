// Generated macro for TryLockResult (type)
macro_rules! Depcrate_sync_nonpoisonTryLockResult {
() => {
// Module: crate::sync::nonpoison
// Provides: {"TryLockResult"}
// Dependencies: {}
# [doc = " A type alias for the result of a nonblocking locking method."] # [unstable (feature = "sync_nonpoison" , issue = "134645")] pub type TryLockResult < Guard > = Result < Guard , WouldBlock > ;
};
}
