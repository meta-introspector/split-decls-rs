// Generated macro for impl_20 (impl)
macro_rules! Depcrate_complex_dictionaryimpl_20 {
() => {
// Module: crate::complex::dictionary
// Provides: {"impl_20"}
// Dependencies: {}
impl < 'l > DictionarySegmenter < 'l > { pub (super) fn new (dict : & 'l UCharDictionaryBreakData < 'l > , grapheme : GraphemeClusterSegmenterBorrowed < 'l > ,) -> Self { Self { dict , grapheme } } # [doc = " Create a dictionary based break iterator for an `str` (a UTF-8 string)."] pub (super) fn segment_str (& 'l self , input : & 'l str) -> impl Iterator < Item = usize > + 'l { let grapheme_iter = self . grapheme . segment_str (input) ; DictionaryBreakIterator :: < char , GraphemeClusterBreakIterator < Utf8 > > { trie : Char16Trie :: new (self . dict . trie_data . clone ()) , iter : input . char_indices () , len : input . len () , grapheme_iter , } } # [doc = " Create a dictionary based break iterator for a UTF-16 string."] pub (super) fn segment_utf16 (& 'l self , input : & 'l [u16]) -> impl Iterator < Item = usize > + 'l { let grapheme_iter = self . grapheme . segment_utf16 (input) ; DictionaryBreakIterator :: < u32 , GraphemeClusterBreakIterator < Utf16 > > { trie : Char16Trie :: new (self . dict . trie_data . clone ()) , iter : Utf16Indices :: new (input) , len : input . len () , grapheme_iter , } } }
};
}
