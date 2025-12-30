// Generated macro for Integer (trait)
macro_rules! Depcrate_parsing_shimInteger {
() => {
// Module: crate::parsing::shim
// Provides: {"Integer"}
// Dependencies: {}
# [doc = " Marker trait for all integer types, including `NonZero*`"] pub (crate) trait Integer : Sized { # [allow (clippy :: missing_docs_in_private_items)] fn parse_bytes (src : & [u8]) -> Option < Self > ; }
};
}
