// Generated macro for impl_61 (impl)
macro_rules! Depcrate_named_valuesimpl_61 {
() => {
// Module: crate::named_values
// Provides: {"impl_61"}
// Dependencies: {}
impl < 'a , 'b > Iterator for Iter < 'a , 'b > { type Item = (& 'b NamedField < 'a > , & 'b Value < 'a >) ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (move | (i , field) | (field , & self . values [i])) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
