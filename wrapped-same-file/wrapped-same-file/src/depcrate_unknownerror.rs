// Generated macro for error (function)
macro_rules! Depcrate_unknownerror {
() => {
// Module: crate::unknown
// Provides: {"error"}
// Dependencies: {}
fn error < T > () -> io :: Result < T > { Err (io :: Error :: new (io :: ErrorKind :: Other , ERROR_MESSAGE)) }
};
}
