// Generated macro for impl_90 (impl)
macro_rules! Depcrateimpl_90 {
() => {
// Module: crate
// Provides: {"impl_90"}
// Dependencies: {}
impl < 'a , T > DoubleEndedIterator for Drain < 'a , T > { fn next_back (& mut self) -> Option < T > { self . iter . next_back () . map (| x | unsafe { ptr :: read (x) }) } }
};
}
