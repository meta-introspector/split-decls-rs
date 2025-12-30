// Generated macro for AsciiMode (enum)
macro_rules! Depcrate_optionsAsciiMode {
() => {
// Module: crate::options
// Provides: {"AsciiMode"}
// Dependencies: {}
# [doc = " Whether to support non-ASCII data in the ZeroTrie."] # [derive (Copy , Clone)] pub (crate) enum AsciiMode { # [doc = " Support only ASCII, returning an error if non-ASCII is found."] AsciiOnly , # [doc = " Support all data, creating span nodes for non-ASCII bytes."] BinarySpans , }
};
}
