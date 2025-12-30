// Generated macro for build_parser (function)
macro_rules! Depcrate_parse_macrosbuild_parser {
() => {
// Module: crate::parse::macros
// Provides: {"build_parser"}
// Dependencies: {}
fn build_parser < 'a > (context : & RewriteContext < 'a > , tokens : TokenStream) -> Parser < 'a > { build_stream_parser (context . psess . inner () , tokens) }
};
}
