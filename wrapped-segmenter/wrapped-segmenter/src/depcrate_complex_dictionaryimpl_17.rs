// Generated macro for impl_17 (impl)
macro_rules! Depcrate_complex_dictionaryimpl_17 {
() => {
// Module: crate::complex::dictionary
// Provides: {"impl_17"}
// Dependencies: {}
impl DictionaryType for u32 { type IterAttr < 's > = Utf16Indices < 's > ; type CharType = u32 ; fn to_char (c : u32) -> char { char :: from_u32 (c) . unwrap_or (char :: REPLACEMENT_CHARACTER) } fn char_len (c : u32) -> usize { if c >= 0x10000 { 2 } else { 1 } } }
};
}
