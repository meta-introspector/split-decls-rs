// Generated macro for impl_62 (impl)
macro_rules! Depcrateimpl_62 {
() => {
// Module: crate
// Provides: {"impl_62"}
// Dependencies: {}
impl < T > IntoIterator for ThinVec < T > { type Item = T ; type IntoIter = IntoIter < T > ; fn into_iter (self) -> IntoIter < T > { IntoIter { vec : self , start : 0 , } } }
};
}
