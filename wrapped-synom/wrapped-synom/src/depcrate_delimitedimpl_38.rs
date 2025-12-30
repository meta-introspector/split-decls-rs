// Generated macro for impl_38 (impl)
macro_rules! Depcrate_delimitedimpl_38 {
() => {
// Module: crate::delimited
// Provides: {"impl_38"}
// Dependencies: {}
impl < 'a , T , D > Iterator for Items < 'a , T , D > { type Item = & 'a T ; fn next (& mut self) -> Option < & 'a T > { self . inner . next () . map (| pair | & pair . 0) } }
};
}
