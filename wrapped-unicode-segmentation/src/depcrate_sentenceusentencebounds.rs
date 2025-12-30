// Generated macro for USentenceBounds (struct)
macro_rules! Depcrate_sentenceUSentenceBounds {
() => {
// Module: crate::sentence
// Provides: {"USentenceBounds"}
// Dependencies: {}
# [doc = " External iterator for a string's"] # [doc = " [sentence boundaries](http://www.unicode.org/reports/tr29/#Sentence_Boundaries)."] # [doc = ""] # [doc = " This struct is created by the [`split_sentence_bounds`] method on the [`UnicodeSegmentation`]"] # [doc = " trait. See its documentation for more."] # [doc = ""] # [doc = " [`split_sentence_bounds`]: trait.UnicodeSegmentation.html#tymethod.split_sentence_bounds"] # [doc = " [`UnicodeSegmentation`]: trait.UnicodeSegmentation.html"] # [derive (Debug , Clone)] pub struct USentenceBounds < 'a > { iter : fwd :: SentenceBreaks < 'a > , sentence_start : Option < usize > , }
};
}
