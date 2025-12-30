// Generated macro for Utf8CharsEx (trait)
macro_rules! DepcrateUtf8CharsEx {
() => {
// Module: crate
// Provides: {"Utf8CharsEx"}
// Dependencies: {}
# [doc = " Convenience trait that adds `chars()` and `char_indices()` methods"] # [doc = " similar to the ones on string slices to byte slices."] pub trait Utf8CharsEx { fn chars (& self) -> Utf8Chars < '_ > ; fn char_indices (& self) -> Utf8CharIndices < '_ > ; }
};
}
