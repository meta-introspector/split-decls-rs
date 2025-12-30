// Generated macro for impl_28 (impl)
macro_rules! Depcrate_complex_languageimpl_28 {
() => {
// Module: crate::complex::language
// Provides: {"impl_28"}
// Dependencies: {}
impl < 's > Iterator for LanguageIterator < 's > { type Item = (& 's str , Language) ; fn next (& mut self) -> Option < Self :: Item > { let mut indices = self . rest . char_indices () ; let lang = get_language (indices . next () ? . 1 as u32) ; match indices . find (| & (_ , ch) | get_language (ch as u32) != lang) { Some ((i , _)) => { let (result , rest) = self . rest . split_at (i) ; self . rest = rest ; Some ((result , lang)) } None => Some ((core :: mem :: take (& mut self . rest) , lang)) , } } }
};
}
