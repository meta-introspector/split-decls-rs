// Generated macro for impl_70 (impl)
macro_rules! Depcrate_diffimpl_70 {
() => {
// Module: crate::diff
// Provides: {"impl_70"}
// Dependencies: {}
impl std :: ops :: Sub for SignedDuration { type Output = SignedDuration ; fn sub (self , rhs : SignedDuration) -> SignedDuration { SignedDuration :: from_nanos (self . as_nanos () - rhs . as_nanos ()) } }
};
}
