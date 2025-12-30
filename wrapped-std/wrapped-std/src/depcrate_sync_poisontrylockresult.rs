// Generated macro for TryLockResult (type)
macro_rules! Depcrate_sync_poisonTryLockResult {
() => {
// Module: crate::sync::poison
// Provides: {"TryLockResult"}
// Dependencies: {}
# [doc = " A type alias for the result of a nonblocking locking method."] # [doc = ""] # [doc = " For more information, see [`LockResult`]. A `TryLockResult` doesn't"] # [doc = " necessarily hold the associated guard in the [`Err`] type as the lock might not"] # [doc = " have been acquired for other reasons."] # [stable (feature = "rust1" , since = "1.0.0")] pub type TryLockResult < Guard > = Result < Guard , TryLockError < Guard > > ;
};
}
