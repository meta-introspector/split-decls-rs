// Generated macro for impl_8 (impl)
macro_rules! Depcrate_indicesimpl_8 {
() => {
// Module: crate::indices
// Provides: {"impl_8"}
// Dependencies: {}
impl < 'a > Utf16CharIndices < 'a > { # [inline (always)] # [doc = " Creates the iterator from a `u16` slice."] pub fn new (code_units : & 'a [u16]) -> Self { Utf16CharIndices :: < 'a > { front_offset : 0 , iter : Utf16Chars :: new (code_units) , } } # [doc = " Views the underlying data as a subslice of the original data."] # [doc = ""] # [doc = " This has the same lifetime as the original slice, and so the"] # [doc = " iterator can continue to be used while this exists."] # [must_use] # [inline] pub fn as_slice (& self) -> & 'a [u16] { self . iter . as_slice () } # [doc = " Returns the code unit position of the next character, or the length"] # [doc = " of the underlying string if there are no more characters."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use utf16_iter::Utf16CharsEx;"] # [doc = " let mut chars = [0xD83Eu16, 0xDD73u16, 0x697Du16].char_indices();"] # [doc = ""] # [doc = " assert_eq!(chars.offset(), 0);"] # [doc = " assert_eq!(chars.next(), Some((0, '🥳')));"] # [doc = ""] # [doc = " assert_eq!(chars.offset(), 2);"] # [doc = " assert_eq!(chars.next(), Some((2, '楽')));"] # [doc = ""] # [doc = " assert_eq!(chars.offset(), 3);"] # [doc = " assert_eq!(chars.next(), None);"] # [doc = " ```"] # [inline] # [must_use] pub fn offset (& self) -> usize { self . front_offset } }
};
}
