// Generated macro for TreeErrorContext (struct)
macro_rules! Depcrate_errorTreeErrorContext {
() => {
// Module: crate::error
// Provides: {"TreeErrorContext"}
// Dependencies: {}
# [doc = " See [`TreeErrorFrame::Context`], [`AddContext::add_context`]"] # [derive (Debug)] # [cfg (feature = "std")] pub struct TreeErrorContext < I , C = StrContext > { # [doc = " Parsed input, at the location where the error occurred"] pub input : I , # [doc = " See [`AddContext::add_context`]"] pub context : C , }
};
}
