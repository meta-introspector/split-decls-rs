// Generated macro for impl_18 (impl)
macro_rules! Depcrate_arrayimpl_18 {
() => {
// Module: crate::array
// Provides: {"impl_18"}
// Dependencies: {}
impl < 's > IntoIterator for & 's Array { type Item = & 's Value ; type IntoIter = ArrayIter < 's > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
