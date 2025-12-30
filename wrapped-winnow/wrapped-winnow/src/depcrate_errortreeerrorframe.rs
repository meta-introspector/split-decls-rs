// Generated macro for TreeErrorFrame (enum)
macro_rules! Depcrate_errorTreeErrorFrame {
() => {
// Module: crate::error
// Provides: {"TreeErrorFrame"}
// Dependencies: {}
# [doc = " See [`TreeError::Stack`]"] # [derive (Debug)] # [cfg (feature = "std")] pub enum TreeErrorFrame < I , C = StrContext > { # [doc = " See [`ParserError::append`]"] Kind (TreeErrorBase < I >) , # [doc = " See [`AddContext::add_context`]"] Context (TreeErrorContext < I , C >) , }
};
}
