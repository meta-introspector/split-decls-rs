// Generated macro for impl_125 (impl)
macro_rules! Depcrate_inherentimpl_125 {
() => {
// Module: crate::inherent
// Provides: {"impl_125"}
// Dependencies: {}
impl < 'a , T : Copy > SliceLike for & 'a [T] { type Item = T ; type IntoIter = std :: iter :: Copied < std :: slice :: Iter < 'a , T > > ; fn iter (self) -> Self :: IntoIter { self . iter () . copied () } fn as_slice (& self) -> & [Self :: Item] { * self } }
};
}
