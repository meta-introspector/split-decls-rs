// Generated macro for echo_back_stdin (function)
macro_rules! Depcrate_formattingecho_back_stdin {
() => {
// Module: crate::formatting
// Provides: {"echo_back_stdin"}
// Dependencies: {}
fn echo_back_stdin (input : & str) -> Result < FormatReport , ErrorKind > { if let Err (e) = io :: stdout () . write_all (input . as_bytes ()) { return Err (From :: from (e)) ; } Ok (FormatReport :: new ()) }
};
}
