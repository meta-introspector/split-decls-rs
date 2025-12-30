// Generated macro for impl_31 (impl)
macro_rules! Depcrate_complex_languageimpl_31 {
() => {
// Module: crate::complex::language
// Provides: {"impl_31"}
// Dependencies: {}
impl < 's > Iterator for LanguageIteratorUtf16 < 's > { type Item = (& 's [u16] , Language) ; fn next (& mut self) -> Option < Self :: Item > { let lang = get_language (* self . rest . first () ? as u32) ; match self . rest . iter () . position (| & ch | get_language (ch as u32) != lang) { Some (i) => { let (result , rest) = self . rest . split_at (i) ; self . rest = rest ; Some ((result , lang)) } None => Some ((core :: mem :: take (& mut self . rest) , lang)) , } } }
};
}
