// Generated macro for LockResult (type)
macro_rules! Depcrate_sync_poisonLockResult {
() => {
// Module: crate::sync::poison
// Provides: {"LockResult"}
// Dependencies: {}
# [doc = " A type alias for the result of a lock method which can be poisoned."] # [doc = ""] # [doc = " The [`Ok`] variant of this result indicates that the primitive was not"] # [doc = " poisoned, and the operation result is contained within. The [`Err`] variant indicates"] # [doc = " that the primitive was poisoned. Note that the [`Err`] variant *also* carries"] # [doc = " an associated value assigned by the lock method, and it can be acquired through the"] # [doc = " [`into_inner`] method. The semantics of the associated value depends on the corresponding"] # [doc = " lock method."] # [doc = ""] # [doc = " [`into_inner`]: PoisonError::into_inner"] # [stable (feature = "rust1" , since = "1.0.0")] pub type LockResult < T > = Result < T , PoisonError < T > > ;
};
}
