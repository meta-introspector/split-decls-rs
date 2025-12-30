// Generated macro for Utf16Indices (struct)
macro_rules! Depcrate_indicesUtf16Indices {
() => {
// Module: crate::indices
// Provides: {"Utf16Indices"}
// Dependencies: {}
# [doc = " Similar to [`core::str::CharIndices`] for UTF-16 strings, represented as `[u16]`."] # [doc = ""] # [doc = " Contrary to [`core::str::CharIndices`], the second element of the"] # [doc = " [`Iterator::Item`] is a Unicode code point represented by a [`u32`],"] # [doc = " rather than a Unicode scalar value represented by a [`char`], because this"] # [doc = " iterator preserves unpaired surrogates."] # [derive (Clone , Debug)] pub struct Utf16Indices < 'a > { front_offset : usize , iter : & 'a [u16] , }
};
}
