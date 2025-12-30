// Generated macro for dump_tokens (function)
macro_rules! Depcrate_debugdump_tokens {
() => {
// Module: crate::debug
// Provides: {"dump_tokens"}
// Dependencies: {}
pub (crate) fn dump_tokens (input_name : impl ToString , tokens : TokenStream) -> TokenStream { if debug_enabled (input_name) { let token_string = tokens . to_string () ; let _ : Result < () , () > = Command :: new ("rustfmt") . arg ("--emit=stdout") . stdin (Stdio :: piped ()) . spawn () . and_then (| mut rustfmt | { rustfmt . stdin . take () . unwrap () . write_all (token_string . as_bytes ()) ? ; rustfmt . wait_with_output () }) . map (| output | eprintln ! ("{}" , String :: from_utf8_lossy (& output . stdout))) . or_else (| _ | { eprintln ! ("{token_string}") ; Ok (()) }) ; } tokens }
};
}
