// Generated macro for impl_3316 (impl)
macro_rules! Depcrate_sync_poisonimpl_3316 {
() => {
// Module: crate::sync::poison
// Provides: {"impl_3316"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T > Error for TryLockError < T > { # [allow (deprecated)] fn cause (& self) -> Option < & dyn Error > { match * self { # [cfg (panic = "unwind")] TryLockError :: Poisoned (ref p) => Some (p) , # [cfg (not (panic = "unwind"))] TryLockError :: Poisoned (ref p) => match p . _never { } , _ => None , } } }
};
}
