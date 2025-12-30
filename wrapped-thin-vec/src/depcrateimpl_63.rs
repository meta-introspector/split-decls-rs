// Generated macro for impl_63 (impl)
macro_rules! Depcrateimpl_63 {
() => {
// Module: crate
// Provides: {"impl_63"}
// Dependencies: {}
impl < 'a , T > IntoIterator for & 'a ThinVec < T > { type Item = & 'a T ; type IntoIter = slice :: Iter < 'a , T > ; fn into_iter (self) -> slice :: Iter < 'a , T > { self . iter () } }
};
}
