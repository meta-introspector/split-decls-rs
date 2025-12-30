// Generated macro for read_stdin (function)
macro_rules! Depcrateread_stdin {
() => {
// Module: crate
// Provides: {"read_stdin"}
// Dependencies: {}
fn read_stdin () -> io :: Result < String > { let mut buf = String :: new () ; io :: stdin () . lock () . read_to_string (& mut buf) ? ; Ok (buf) }
};
}
