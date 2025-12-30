// Generated macro for impl_27 (impl)
macro_rules! Depcrate_word_splittersimpl_27 {
() => {
// Module: crate::word_splitters
// Provides: {"impl_27"}
// Dependencies: {}
impl WordSplitter { # [doc = " Return all possible indices where `word` can be split."] # [doc = ""] # [doc = " The indices are in the range `0..word.len()`. They point to"] # [doc = " the index _after_ the split point, i.e., after `-` if"] # [doc = " splitting on hyphens. This way, `word.split_at(idx)` will"] # [doc = " break the word into two well-formed pieces."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use textwrap::WordSplitter;"] # [doc = " assert_eq!(WordSplitter::NoHyphenation.split_points(\"cannot-be-split\"), vec![]);"] # [doc = " assert_eq!(WordSplitter::HyphenSplitter.split_points(\"can-be-split\"), vec![4, 7]);"] # [doc = " assert_eq!(WordSplitter::Custom(|word| vec![word.len()/2]).split_points(\"middle\"), vec![3]);"] # [doc = " ```"] pub fn split_points (& self , word : & str) -> Vec < usize > { match self { WordSplitter :: NoHyphenation => Vec :: new () , WordSplitter :: HyphenSplitter => { let mut splits = Vec :: new () ; for (idx , _) in word . match_indices ('-') { let prev = word [.. idx] . chars () . next_back () ; let next = word [idx + 1 ..] . chars () . next () ; if prev . filter (| ch | ch . is_alphanumeric ()) . is_some () && next . filter (| ch | ch . is_alphanumeric ()) . is_some () { splits . push (idx + 1) ; } } splits } WordSplitter :: Custom (splitter_func) => splitter_func (word) , # [cfg (feature = "hyphenation")] WordSplitter :: Hyphenation (dictionary) => { use hyphenation :: Hyphenator ; dictionary . hyphenate (word) . breaks } } } }
};
}
