// Generated macro for Error (struct)
macro_rules! Depcrate_processorError {
() => {
// Module: crate::processor
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error type returned if a [`Processor`] fails."] # [derive (Error , Debug)] # [error ("{source}")] pub struct Error { # [doc = " The recoverable [`Tree`] type that couldn't be processed."] pub tree : Tree , source : Box < dyn error :: Error + Send + Sync > , }
};
}
