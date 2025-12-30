// Generated macro for impl_8 (impl)
macro_rules! Depcrate_indicesimpl_8 {
() => {
// Module: crate::indices
// Provides: {"impl_8"}
// Dependencies: {}
impl < 'a > Utf8CharIndices < 'a > { # [inline (always)] # [doc = " Creates the iterator from a byte slice."] pub fn new (bytes : & 'a [u8]) -> Self { Utf8CharIndices :: < 'a > { front_offset : 0 , iter : Utf8Chars :: new (bytes) , } } # [doc = " Views the underlying data as a subslice of the original data."] # [doc = ""] # [doc = " This has the same lifetime as the original slice, and so the"] # [doc = " iterator can continue to be used while this exists."] # [must_use] # [inline] pub fn as_slice (& self) -> & 'a [u8] { self . iter . as_slice () } # [doc = " Returns the byte position of the next character, or the length"] # [doc = " of the underlying string if there are no more characters."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use utf8_iter::Utf8CharsEx;"] # [doc = " let mut chars = \"a楽\".as_bytes().char_indices();"] # [doc = ""] # [doc = " assert_eq!(chars.offset(), 0);"] # [doc = " assert_eq!(chars.next(), Some((0, 'a')));"] # [doc = ""] # [doc = " assert_eq!(chars.offset(), 1);"] # [doc = " assert_eq!(chars.next(), Some((1, '楽')));"] # [doc = ""] # [doc = " assert_eq!(chars.offset(), 4);"] # [doc = " assert_eq!(chars.next(), None);"] # [doc = " ```"] # [inline] # [must_use] pub fn offset (& self) -> usize { self . front_offset } }
};
}
