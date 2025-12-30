// Generated macro for format_code (function)
macro_rules! Depcrateformat_code {
() => {
// Module: crate
// Provides: {"format_code"}
// Dependencies: {}
pub fn format_code (mut output : impl std :: io :: Write , input : impl std :: fmt :: Display ,) -> std :: io :: Result < () > { let proc = Command :: new ("rustfmt") . stdin (Stdio :: piped ()) . stdout (Stdio :: piped ()) . spawn () ? ; write ! (proc . stdin . as_ref () . unwrap () , "{input}") ? ; output . write_all (proc . wait_with_output () ? . stdout . as_slice ()) }
};
}
