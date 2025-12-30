// Generated macro for impl_60 (impl)
macro_rules! Depcrate_wordimpl_60 {
() => {
// Module: crate::word
// Provides: {"impl_60"}
// Dependencies: {}
impl < 'a > Iterator for UnicodeWords < 'a > { type Item = & 'a str ; # [inline] fn next (& mut self) -> Option < Self :: Item > { match & mut self . inner { WordsIter :: Ascii (i) => i . next () , WordsIter :: Unicode (i) => i . next () , } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { match & self . inner { WordsIter :: Ascii (i) => i . size_hint () , WordsIter :: Unicode (i) => i . size_hint () , } } }
};
}
