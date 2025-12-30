// Generated macro for Latin1Indices (struct)
macro_rules! Depcrate_indicesLatin1Indices {
() => {
// Module: crate::indices
// Provides: {"Latin1Indices"}
// Dependencies: {}
# [doc = " Similar to [`core::str::CharIndices`] for Latin-1 strings, represented as `[u8]`."] # [doc = ""] # [doc = " Contrary to [`core::str::CharIndices`], the second element of the"] # [doc = " [`Iterator::Item`] is a [`u8`], representing a Unicode scalar value in the"] # [doc = " range U+0000–U+00FF."] # [derive (Clone , Debug)] pub struct Latin1Indices < 'a > { front_offset : usize , iter : & 'a [u8] , }
};
}
