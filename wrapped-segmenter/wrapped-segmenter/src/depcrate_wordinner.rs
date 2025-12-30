// Generated macro for inner (module)
macro_rules! Depcrate_wordinner {
() => {
// Module: crate::word
// Provides: {"inner"}
// Dependencies: {}
# [doc = " Hide ULE type"] pub (crate) mod inner { # [doc = " The word type tag that is returned by [`WordBreakIterator::word_type()`]."] # [doc = ""] # [doc = " [`WordBreakIterator::word_type()`]: super::WordBreakIterator::word_type"] # [non_exhaustive] # [derive (Copy , Clone , PartialEq , Debug)] # [repr (u8)] # [zerovec :: make_ule (WordTypeULE)] pub enum WordType { # [doc = " No category tag."] None = 0 , # [doc = " Number category tag."] Number = 1 , # [doc = " Letter category tag, including CJK."] Letter = 2 , } }
};
}
