// Generated macro for impl_3314 (impl)
macro_rules! Depcrate_sync_poisonimpl_3314 {
() => {
// Module: crate::sync::poison
// Provides: {"impl_3314"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T > fmt :: Debug for TryLockError < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { # [cfg (panic = "unwind")] TryLockError :: Poisoned (..) => "Poisoned(..)" . fmt (f) , # [cfg (not (panic = "unwind"))] TryLockError :: Poisoned (ref p) => match p . _never { } , TryLockError :: WouldBlock => "WouldBlock" . fmt (f) , } } }
};
}
