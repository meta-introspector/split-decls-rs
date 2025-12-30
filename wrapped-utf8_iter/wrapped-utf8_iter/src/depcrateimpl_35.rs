// Generated macro for impl_35 (impl)
macro_rules! Depcrateimpl_35 {
() => {
// Module: crate
// Provides: {"impl_35"}
// Dependencies: {}
impl Utf8CharsEx for [u8] { # [doc = " Convenience method for creating an UTF-8 iterator"] # [doc = " for the slice."] # [inline] fn chars (& self) -> Utf8Chars < '_ > { Utf8Chars :: new (self) } # [doc = " Convenience method for creating a byte index and"] # [doc = " UTF-8 iterator for the slice."] # [inline] fn char_indices (& self) -> Utf8CharIndices < '_ > { Utf8CharIndices :: new (self) } }
};
}
