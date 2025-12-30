// Generated macro for impl_127 (impl)
macro_rules! Depcrate_inherentimpl_127 {
() => {
// Module: crate::inherent
// Provides: {"impl_127"}
// Dependencies: {}
impl < 'a , S : SliceLike > SliceLike for & 'a S { type Item = S :: Item ; type IntoIter = S :: IntoIter ; fn iter (self) -> Self :: IntoIter { (* self) . iter () } fn as_slice (& self) -> & [Self :: Item] { (* self) . as_slice () } }
};
}
