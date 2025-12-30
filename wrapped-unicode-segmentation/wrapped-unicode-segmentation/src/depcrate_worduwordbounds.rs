// Generated macro for UWordBounds (struct)
macro_rules! Depcrate_wordUWordBounds {
() => {
// Module: crate::word
// Provides: {"UWordBounds"}
// Dependencies: {}
# [doc = " External iterator for a string's"] # [doc = " [word boundaries](http://www.unicode.org/reports/tr29/#Word_Boundaries)."] # [doc = ""] # [doc = " This struct is created by the [`split_word_bounds`] method on the [`UnicodeSegmentation`]"] # [doc = " trait. See its documentation for more."] # [doc = ""] # [doc = " [`split_word_bounds`]: trait.UnicodeSegmentation.html#tymethod.split_word_bounds"] # [doc = " [`UnicodeSegmentation`]: trait.UnicodeSegmentation.html"] # [derive (Debug , Clone)] pub struct UWordBounds < 'a > { string : & 'a str , cat : Option < WordCat > , catb : Option < WordCat > , }
};
}
