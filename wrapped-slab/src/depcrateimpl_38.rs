// Generated macro for impl_38 (impl)
macro_rules! Depcrateimpl_38 {
() => {
// Module: crate
// Provides: {"impl_38"}
// Dependencies: {}
impl < T > IntoIterator for Slab < T > { type Item = (usize , T) ; type IntoIter = IntoIter < T > ; fn into_iter (self) -> IntoIter < T > { IntoIter { entries : self . entries . into_iter () . enumerate () , len : self . len , } } }
};
}
