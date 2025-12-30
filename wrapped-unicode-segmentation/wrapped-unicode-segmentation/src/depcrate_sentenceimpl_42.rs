// Generated macro for impl_42 (impl)
macro_rules! Depcrate_sentenceimpl_42 {
() => {
// Module: crate::sentence
// Provides: {"impl_42"}
// Dependencies: {}
impl < 'a > Iterator for USentenceBounds < 'a > { type Item = & 'a str ; # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let (lower , upper) = self . iter . size_hint () ; (cmp :: max (0 , lower - 1) , upper . map (| u | cmp :: max (0 , u - 1))) } # [inline] fn next (& mut self) -> Option < & 'a str > { if self . sentence_start . is_none () { if let Some (start_pos) = self . iter . next () { self . sentence_start = Some (start_pos) } else { return None ; } } if let Some (break_pos) = self . iter . next () { let start_pos = self . sentence_start . unwrap () ; let sentence = & self . iter . string [start_pos .. break_pos] ; self . sentence_start = Some (break_pos) ; Some (sentence) } else { None } } }
};
}
