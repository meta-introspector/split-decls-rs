// Generated macro for DictionaryType (trait)
macro_rules! Depcrate_complex_dictionaryDictionaryType {
() => {
// Module: crate::complex::dictionary
// Provides: {"DictionaryType"}
// Dependencies: {}
# [doc = " A trait for dictionary based iterator"] trait DictionaryType { # [doc = " The iterator over characters."] type IterAttr < 's > : Iterator < Item = (usize , Self :: CharType) > + Clone ; # [doc = " The character type."] type CharType : Copy + Into < u32 > ; fn to_char (c : Self :: CharType) -> char ; fn char_len (c : Self :: CharType) -> usize ; }
};
}
