// Generated macro for Utf16CharsEx (trait)
macro_rules! DepcrateUtf16CharsEx {
() => {
// Module: crate
// Provides: {"Utf16CharsEx"}
// Dependencies: {}
# [doc = " Convenience trait that adds `chars()` and `char_indices()` methods"] # [doc = " similar to the ones on string slices to `u16` slices."] pub trait Utf16CharsEx { fn chars (& self) -> Utf16Chars < '_ > ; fn char_indices (& self) -> Utf16CharIndices < '_ > ; }
};
}
