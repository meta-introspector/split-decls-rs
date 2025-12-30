// Generated macro for impl_31 (impl)
macro_rules! Depcrate_delimitedimpl_31 {
() => {
// Module: crate::delimited
// Provides: {"impl_31"}
// Dependencies: {}
impl < T , D > IntoIterator for Delimited < T , D > { type Item = Element < T , D > ; type IntoIter = IntoIter < T , D > ; fn into_iter (self) -> IntoIter < T , D > { IntoIter { inner : self . inner . into_iter () } } }
};
}
