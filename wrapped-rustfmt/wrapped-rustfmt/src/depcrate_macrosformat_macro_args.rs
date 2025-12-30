// Generated macro for format_macro_args (function)
macro_rules! Depcrate_macrosformat_macro_args {
() => {
// Module: crate::macros
// Provides: {"format_macro_args"}
// Dependencies: {}
fn format_macro_args (context : & RewriteContext < '_ > , token_stream : TokenStream , shape : Shape ,) -> RewriteResult { let span = span_for_token_stream (& token_stream) ; if ! context . config . format_macro_matchers () { return Ok (match span { Some (span) => context . snippet (span) . to_owned () , None => String :: new () , }) ; } let parsed_args = MacroArgParser :: new () . parse (token_stream) . macro_error (MacroErrorKind :: ParseFailure , span . unwrap ()) ? ; wrap_macro_args (context , & parsed_args , shape) }
};
}
