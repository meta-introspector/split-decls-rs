// Generated macro for DictionaryBreakIterator (struct)
macro_rules! Depcrate_complex_dictionaryDictionaryBreakIterator {
() => {
// Module: crate::complex::dictionary
// Provides: {"DictionaryBreakIterator"}
// Dependencies: {}
struct DictionaryBreakIterator < 'l , 's , Y : DictionaryType + ? Sized , X : Iterator < Item = usize > + ? Sized , > { trie : Char16Trie < 'l > , iter : Y :: IterAttr < 's > , len : usize , grapheme_iter : X , }
};
}
