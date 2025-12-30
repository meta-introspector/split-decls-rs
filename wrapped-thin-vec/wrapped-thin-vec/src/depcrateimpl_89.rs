// Generated macro for impl_89 (impl)
macro_rules! Depcrateimpl_89 {
() => {
// Module: crate
// Provides: {"impl_89"}
// Dependencies: {}
impl < 'a , T > Iterator for Drain < 'a , T > { type Item = T ; fn next (& mut self) -> Option < T > { self . iter . next () . map (| x | unsafe { ptr :: read (x) }) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
