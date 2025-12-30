// Generated macro for impl_43 (impl)
macro_rules! Depcrate_sentenceimpl_43 {
() => {
// Module: crate::sentence
// Provides: {"impl_43"}
// Dependencies: {}
impl < 'a > Iterator for USentenceBoundIndices < 'a > { type Item = (usize , & 'a str) ; # [inline] fn next (& mut self) -> Option < (usize , & 'a str) > { self . iter . next () . map (| s | (s . as_ptr () as usize - self . start_offset , s)) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
