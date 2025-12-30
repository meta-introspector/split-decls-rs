// Generated macro for macro_994 (macro)
macro_rules! Depcrate_non_ascii_identsmacro_994 {
() => {
// Module: crate::non_ascii_idents
// Provides: {"macro_994"}
// Dependencies: {}
declare_lint ! { # [doc = " The `non_ascii_idents` lint detects non-ASCII identifiers."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " # #![allow(unused)]"] # [doc = " #![deny(non_ascii_idents)]"] # [doc = " fn main() {"] # [doc = "     let föö = 1;"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " This lint allows projects that wish to retain the limit of only using"] # [doc = " ASCII characters to switch this lint to \"forbid\" (for example to ease"] # [doc = " collaboration or for security reasons)."] # [doc = " See [RFC 2457] for more details."] # [doc = ""] # [doc = " [RFC 2457]: https://github.com/rust-lang/rfcs/blob/master/text/2457-non-ascii-idents.md"] pub NON_ASCII_IDENTS , Allow , "detects non-ASCII identifiers" , crate_level_only }
};
}
