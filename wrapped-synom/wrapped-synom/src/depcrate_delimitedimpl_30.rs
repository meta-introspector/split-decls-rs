// Generated macro for impl_30 (impl)
macro_rules! Depcrate_delimitedimpl_30 {
() => {
// Module: crate::delimited
// Provides: {"impl_30"}
// Dependencies: {}
impl < 'a , T , D > IntoIterator for & 'a Delimited < T , D > { type Item = Element < & 'a T , & 'a D > ; type IntoIter = Iter < 'a , T , D > ; fn into_iter (self) -> Iter < 'a , T , D > { < Delimited < T , D > > :: iter (self) } }
};
}
