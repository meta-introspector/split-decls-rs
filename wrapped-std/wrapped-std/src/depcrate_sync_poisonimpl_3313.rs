// Generated macro for impl_3313 (impl)
macro_rules! Depcrate_sync_poisonimpl_3313 {
() => {
// Module: crate::sync::poison
// Provides: {"impl_3313"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T > From < PoisonError < T > > for TryLockError < T > { fn from (err : PoisonError < T >) -> TryLockError < T > { TryLockError :: Poisoned (err) } }
};
}
