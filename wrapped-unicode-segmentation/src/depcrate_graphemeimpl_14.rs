// Generated macro for impl_14 (impl)
macro_rules! Depcrate_graphemeimpl_14 {
() => {
// Module: crate::grapheme
// Provides: {"impl_14"}
// Dependencies: {}
impl < 'a > DoubleEndedIterator for GraphemeIndices < 'a > { # [inline] fn next_back (& mut self) -> Option < (usize , & 'a str) > { self . iter . next_back () . map (| s | (s . as_ptr () as usize - self . start_offset , s)) } }
};
}
