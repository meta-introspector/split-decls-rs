// Generated macro for impl_26 (impl)
macro_rules! Depcrate_word_splittersimpl_26 {
() => {
// Module: crate::word_splitters
// Provides: {"impl_26"}
// Dependencies: {}
impl PartialEq < WordSplitter > for WordSplitter { fn eq (& self , other : & WordSplitter) -> bool { match (self , other) { (WordSplitter :: NoHyphenation , WordSplitter :: NoHyphenation) => true , (WordSplitter :: HyphenSplitter , WordSplitter :: HyphenSplitter) => true , # [cfg (feature = "hyphenation")] (WordSplitter :: Hyphenation (this_dict) , WordSplitter :: Hyphenation (other_dict)) => { this_dict . language () == other_dict . language () } (_ , _) => false , } } }
};
}
