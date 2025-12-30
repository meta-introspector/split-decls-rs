// Generated macro for CharULE (struct)
macro_rules! Depcrate_ule_charsCharULE {
() => {
// Module: crate::ule::chars
// Provides: {"CharULE"}
// Dependencies: {}
# [doc = " A u8 array of little-endian data corresponding to a Unicode scalar value."] # [doc = ""] # [doc = " The bytes of a `CharULE` are guaranteed to represent a little-endian-encoded u32 that is a"] # [doc = " valid `char` and can be converted without validation."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Convert a `char` to a `CharULE` and back again:"] # [doc = ""] # [doc = " ```"] # [doc = " use zerovec::ule::{AsULE, CharULE, ULE};"] # [doc = ""] # [doc = " let c1 = '𑄃';"] # [doc = " let ule = c1.to_unaligned();"] # [doc = " assert_eq!(CharULE::slice_as_bytes(&[ule]), &[0x03, 0x11, 0x01]);"] # [doc = " let c2 = char::from_unaligned(ule);"] # [doc = " assert_eq!(c1, c2);"] # [doc = " ```"] # [doc = ""] # [doc = " Attempt to parse invalid bytes to a `CharULE`:"] # [doc = ""] # [doc = " ```"] # [doc = " use zerovec::ule::{CharULE, ULE};"] # [doc = ""] # [doc = " let bytes: &[u8] = &[0xFF, 0xFF, 0xFF, 0xFF];"] # [doc = " CharULE::parse_bytes_to_slice(bytes).expect_err(\"Invalid bytes\");"] # [doc = " ```"] # [repr (transparent)] # [derive (Debug , PartialEq , Eq , Clone , Copy , Hash)] pub struct CharULE ([u8 ; 3]) ;
};
}
