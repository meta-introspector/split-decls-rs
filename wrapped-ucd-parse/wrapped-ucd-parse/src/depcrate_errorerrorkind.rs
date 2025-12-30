// Generated macro for ErrorKind (enum)
macro_rules! Depcrate_errorErrorKind {
() => {
// Module: crate::error
// Provides: {"ErrorKind"}
// Dependencies: {}
# [doc = " The kind of error that occurred while parsing the UCD."] # [derive (Debug)] pub enum ErrorKind { # [doc = " An I/O error."] Io (std :: io :: Error) , # [doc = " A generic parse error."] Parse (String) , }
};
}
