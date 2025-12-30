// Generated macro for impl_3315 (impl)
macro_rules! Depcrate_sync_poisonimpl_3315 {
() => {
// Module: crate::sync::poison
// Provides: {"impl_3315"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T > fmt :: Display for TryLockError < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { # [cfg (panic = "unwind")] TryLockError :: Poisoned (..) => "poisoned lock: another task failed inside" , # [cfg (not (panic = "unwind"))] TryLockError :: Poisoned (ref p) => match p . _never { } , TryLockError :: WouldBlock => "try_lock failed because the operation would block" , } . fmt (f) } }
};
}
