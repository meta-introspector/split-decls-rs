// Generated macro for impl_66 (impl)
macro_rules! Depcrate_diffimpl_66 {
() => {
// Module: crate::diff
// Provides: {"impl_66"}
// Dependencies: {}
impl SignedDuration { pub fn as_nanos (& self) -> i128 { let sign = if self . is_positive { 1 } else { - 1 } ; sign * (self . duration . as_nanos () as i128) } pub fn from_nanos (nanos : i128) -> SignedDuration { let is_positive = nanos >= 0 ; SignedDuration { duration : Duration :: from_nanos (nanos . abs () as u64) , is_positive , } } }
};
}
