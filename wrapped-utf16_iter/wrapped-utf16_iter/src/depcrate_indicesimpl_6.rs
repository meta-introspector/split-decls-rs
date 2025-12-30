// Generated macro for impl_6 (impl)
macro_rules! Depcrate_indicesimpl_6 {
() => {
// Module: crate::indices
// Provides: {"impl_6"}
// Dependencies: {}
impl < 'a > DoubleEndedIterator for Utf16CharIndices < 'a > { # [inline] fn next_back (& mut self) -> Option < (usize , char) > { self . iter . next_back () . map (| ch | { let index = self . front_offset + self . as_slice () . len () ; (index , ch) }) } }
};
}
