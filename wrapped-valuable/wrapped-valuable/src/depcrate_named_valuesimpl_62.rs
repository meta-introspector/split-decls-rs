// Generated macro for impl_62 (impl)
macro_rules! Depcrate_named_valuesimpl_62 {
() => {
// Module: crate::named_values
// Provides: {"impl_62"}
// Dependencies: {}
impl DoubleEndedIterator for Iter < '_ , '_ > { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . next_back () . map (move | (i , field) | (field , & self . values [i])) } }
};
}
