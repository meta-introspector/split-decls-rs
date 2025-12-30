// Generated macro for impl_59 (impl)
macro_rules! Depcrate_named_valuesimpl_59 {
() => {
// Module: crate::named_values
// Provides: {"impl_59"}
// Dependencies: {}
impl < 'a , 'b > IntoIterator for & 'b NamedValues < 'a > { type Item = (& 'b NamedField < 'a > , & 'b Value < 'a >) ; type IntoIter = Iter < 'a , 'b > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
