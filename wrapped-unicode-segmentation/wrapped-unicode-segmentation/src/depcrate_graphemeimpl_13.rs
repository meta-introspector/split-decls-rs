// Generated macro for impl_13 (impl)
macro_rules! Depcrate_graphemeimpl_13 {
() => {
// Module: crate::grapheme
// Provides: {"impl_13"}
// Dependencies: {}
impl < 'a > Iterator for GraphemeIndices < 'a > { type Item = (usize , & 'a str) ; # [inline] fn next (& mut self) -> Option < (usize , & 'a str) > { self . iter . next () . map (| s | (s . as_ptr () as usize - self . start_offset , s)) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
