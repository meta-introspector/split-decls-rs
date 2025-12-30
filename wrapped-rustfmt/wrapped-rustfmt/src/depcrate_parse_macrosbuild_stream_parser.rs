// Generated macro for build_stream_parser (function)
macro_rules! Depcrate_parse_macrosbuild_stream_parser {
() => {
// Module: crate::parse::macros
// Provides: {"build_stream_parser"}
// Dependencies: {}
fn build_stream_parser < 'a > (psess : & 'a ParseSess , tokens : TokenStream) -> Parser < 'a > { Parser :: new (psess , tokens , MACRO_ARGUMENTS) . recovery (Recovery :: Forbidden) }
};
}
