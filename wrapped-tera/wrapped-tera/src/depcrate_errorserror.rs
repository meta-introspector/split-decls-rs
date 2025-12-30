// Generated macro for Error (struct)
macro_rules! Depcrate_errorsError {
() => {
// Module: crate::errors
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The Error type"] # [derive (Debug)] pub struct Error { # [doc = " Kind of error"] pub kind : ErrorKind , source : Option < Box < dyn StdError + Sync + Send > > , }
};
}
