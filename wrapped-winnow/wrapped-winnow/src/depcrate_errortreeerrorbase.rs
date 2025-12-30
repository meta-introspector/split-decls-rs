// Generated macro for TreeErrorBase (struct)
macro_rules! Depcrate_errorTreeErrorBase {
() => {
// Module: crate::error
// Provides: {"TreeErrorBase"}
// Dependencies: {}
# [doc = " See [`TreeErrorFrame::Kind`], [`ParserError::append`]"] # [derive (Debug)] # [cfg (feature = "std")] pub struct TreeErrorBase < I > { # [doc = " Parsed input, at the location where the error occurred"] pub input : I , # [doc = " See [`FromExternalError::from_external_error`]"] pub cause : Option < Box < dyn std :: error :: Error + Send + Sync + 'static > > , }
};
}
