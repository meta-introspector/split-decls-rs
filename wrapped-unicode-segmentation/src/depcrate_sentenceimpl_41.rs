// Generated macro for impl_41 (impl)
macro_rules! Depcrate_sentenceimpl_41 {
() => {
// Module: crate::sentence
// Provides: {"impl_41"}
// Dependencies: {}
impl < 'a > Iterator for UnicodeSentences < 'a > { type Item = & 'a str ; # [inline] fn next (& mut self) -> Option < & 'a str > { self . inner . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
};
}
