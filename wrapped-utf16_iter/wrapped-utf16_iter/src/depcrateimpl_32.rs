// Generated macro for impl_32 (impl)
macro_rules! Depcrateimpl_32 {
() => {
// Module: crate
// Provides: {"impl_32"}
// Dependencies: {}
impl Utf16CharsEx for [u16] { # [doc = " Convenience method for creating an UTF-16 iterator"] # [doc = " for the slice."] # [inline] fn chars (& self) -> Utf16Chars < '_ > { Utf16Chars :: new (self) } # [doc = " Convenience method for creating a code unit index and"] # [doc = " UTF-16 iterator for the slice."] # [inline] fn char_indices (& self) -> Utf16CharIndices < '_ > { Utf16CharIndices :: new (self) } }
};
}
