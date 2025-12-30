// Generated macro for UnicodeSentences (struct)
macro_rules! Depcrate_sentenceUnicodeSentences {
() => {
// Module: crate::sentence
// Provides: {"UnicodeSentences"}
// Dependencies: {}
# [doc = " An iterator over the substrings of a string which, after splitting the string on"] # [doc = " [sentence boundaries](http://www.unicode.org/reports/tr29/#Sentence_Boundaries),"] # [doc = " contain any characters with the"] # [doc = " [Alphabetic](http://unicode.org/reports/tr44/#Alphabetic)"] # [doc = " property, or with"] # [doc = " [General_Category=Number](http://unicode.org/reports/tr44/#General_Category_Values)."] # [doc = ""] # [doc = " This struct is created by the [`unicode_sentences`] method on the [`UnicodeSegmentation`]"] # [doc = " trait. See its documentation for more."] # [doc = ""] # [doc = " [`unicode_sentences`]: trait.UnicodeSegmentation.html#tymethod.unicode_sentences"] # [doc = " [`UnicodeSegmentation`]: trait.UnicodeSegmentation.html"] # [derive (Debug , Clone)] pub struct UnicodeSentences < 'a > { inner : Filter < USentenceBounds < 'a > , fn (& & str) -> bool > , }
};
}
