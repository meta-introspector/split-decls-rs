// Generated macro for Error (enum)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " An error enum for all error types."] # [derive (Debug)] pub enum Error { # [doc = " A [`std::io::Error`]."] Io (std :: io :: Error) , # [doc = " A [`combine::stream::read::Error`]."] Read (combine :: stream :: read :: Error) , # [doc = " A [`combine::error::UnexpectedParse`]."] Parse (combine :: error :: UnexpectedParse) , }
};
}
