// Generated macro for RuleBreakType (trait)
macro_rules! Depcrate_rule_segmenterRuleBreakType {
() => {
// Module: crate::rule_segmenter
// Provides: {"RuleBreakType"}
// Dependencies: {}
# [doc = " A trait allowing for RuleBreakIterator to be generalized to multiple string"] # [doc = " encoding methods and granularity such as grapheme cluster, word, etc."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚫 This trait is sealed; it cannot be implemented by user code. If an API requests an item that implements this"] # [doc = " trait, please consider using a type from the implementors listed below."] # [doc = " </div>"] pub trait RuleBreakType : crate :: private :: Sealed + Sized { # [doc = " The iterator over characters."] type IterAttr < 's > : Iterator < Item = (usize , Self :: CharType) > + Clone + core :: fmt :: Debug ; # [doc = " The character type."] type CharType : Copy + Into < u32 > + core :: fmt :: Debug ; # [doc (hidden)] fn char_len (ch : Self :: CharType) -> usize ; }
};
}
