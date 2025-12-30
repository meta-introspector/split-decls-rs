// Generated macro for impl_67 (impl)
macro_rules! Depcrate_wordimpl_67 {
() => {
// Module: crate::word
// Provides: {"impl_67"}
// Dependencies: {}
impl < 'a > UWordBoundIndices < 'a > { # [inline] # [doc = " View the underlying data (the part yet to be iterated) as a slice of the original string."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use unicode_segmentation::UnicodeSegmentation;"] # [doc = " let mut iter = \"Hello world\".split_word_bound_indices();"] # [doc = " assert_eq!(iter.as_str(), \"Hello world\");"] # [doc = " iter.next();"] # [doc = " assert_eq!(iter.as_str(), \" world\");"] # [doc = " iter.next();"] # [doc = " assert_eq!(iter.as_str(), \"world\");"] # [doc = " ```"] pub fn as_str (& self) -> & 'a str { self . iter . as_str () } }
};
}
