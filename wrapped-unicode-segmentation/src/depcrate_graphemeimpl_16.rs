// Generated macro for impl_16 (impl)
macro_rules! Depcrate_graphemeimpl_16 {
() => {
// Module: crate::grapheme
// Provides: {"impl_16"}
// Dependencies: {}
impl < 'a > Graphemes < 'a > { # [inline] # [doc = " View the underlying data (the part yet to be iterated) as a slice of the original string."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use unicode_segmentation::UnicodeSegmentation;"] # [doc = " let mut iter = \"abc\".graphemes(true);"] # [doc = " assert_eq!(iter.as_str(), \"abc\");"] # [doc = " iter.next();"] # [doc = " assert_eq!(iter.as_str(), \"bc\");"] # [doc = " iter.next();"] # [doc = " iter.next();"] # [doc = " assert_eq!(iter.as_str(), \"\");"] # [doc = " ```"] pub fn as_str (& self) -> & 'a str { & self . string [self . cursor . cur_cursor () .. self . cursor_back . cur_cursor ()] } }
};
}
