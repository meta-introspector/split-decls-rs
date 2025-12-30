// Generated macro for Error (struct)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Represents any kind of error that can occur while parsing the UCD."] # [derive (Debug)] pub struct Error { pub (crate) kind : ErrorKind , pub (crate) line : Option < u64 > , pub (crate) path : Option < PathBuf > , }
};
}
