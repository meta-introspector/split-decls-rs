// Generated macro for impl_68 (impl)
macro_rules! Depcrate_wordimpl_68 {
() => {
// Module: crate::word
// Provides: {"impl_68"}
// Dependencies: {}
impl < 'a > Iterator for UWordBoundIndices < 'a > { type Item = (usize , & 'a str) ; # [inline] fn next (& mut self) -> Option < (usize , & 'a str) > { self . iter . next () . map (| s | (s . as_ptr () as usize - self . start_offset , s)) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
