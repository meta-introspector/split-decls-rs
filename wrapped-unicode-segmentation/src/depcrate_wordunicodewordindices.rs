// Generated macro for UnicodeWordIndices (struct)
macro_rules! Depcrate_wordUnicodeWordIndices {
() => {
// Module: crate::word
// Provides: {"UnicodeWordIndices"}
// Dependencies: {}
# [doc = " An iterator over the substrings of a string which, after splitting the string on"] # [doc = " [word boundaries](http://www.unicode.org/reports/tr29/#Word_Boundaries),"] # [doc = " contain any characters with the"] # [doc = " [Alphabetic](http://unicode.org/reports/tr44/#Alphabetic)"] # [doc = " property, or with"] # [doc = " [General_Category=Number](http://unicode.org/reports/tr44/#General_Category_Values)."] # [doc = " This iterator also provides the byte offsets for each substring."] # [doc = ""] # [doc = " This struct is created by the [`unicode_word_indices`] method on the [`UnicodeSegmentation`] trait. See"] # [doc = " its documentation for more."] # [doc = ""] # [doc = " [`unicode_word_indices`]: trait.UnicodeSegmentation.html#tymethod.unicode_word_indices"] # [doc = " [`UnicodeSegmentation`]: trait.UnicodeSegmentation.html"] # [derive (Debug)] pub struct UnicodeWordIndices < 'a > { inner : IndicesIter < 'a > , }
};
}
