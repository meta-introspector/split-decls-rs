// Generated macro for impl_319 (impl)
macro_rules! Depcrate_tableimpl_319 {
() => {
// Module: crate::table
// Provides: {"impl_319"}
// Dependencies: {}
impl < 's > IntoIterator for & 's Table { type Item = (& 's str , & 's Item) ; type IntoIter = Iter < 's > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
