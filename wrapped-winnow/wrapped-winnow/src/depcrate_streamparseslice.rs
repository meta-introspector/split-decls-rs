// Generated macro for ParseSlice (trait)
macro_rules! Depcrate_streamParseSlice {
() => {
// Module: crate::stream
// Provides: {"ParseSlice"}
// Dependencies: {}
# [doc = " Used to integrate `str`'s `parse()` method"] pub trait ParseSlice < R > { # [doc = " Succeeds if `parse()` succeeded"] # [doc = ""] # [doc = " The byte slice implementation will first convert it to a `&str`, then apply the `parse()`"] # [doc = " function"] fn parse_slice (& self) -> Option < R > ; }
};
}
