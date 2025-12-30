// Generated macro for impl_69 (impl)
macro_rules! Depcrate_wordimpl_69 {
() => {
// Module: crate::word
// Provides: {"impl_69"}
// Dependencies: {}
impl < 'a > DoubleEndedIterator for UWordBoundIndices < 'a > { # [inline] fn next_back (& mut self) -> Option < (usize , & 'a str) > { self . iter . next_back () . map (| s | (s . as_ptr () as usize - self . start_offset , s)) } }
};
}
