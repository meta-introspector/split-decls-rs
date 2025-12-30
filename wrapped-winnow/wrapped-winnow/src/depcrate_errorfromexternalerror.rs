// Generated macro for FromExternalError (trait)
macro_rules! Depcrate_errorFromExternalError {
() => {
// Module: crate::error
// Provides: {"FromExternalError"}
// Dependencies: {}
# [doc = " Create a new error with an external error, from [`std::str::FromStr`]"] # [doc = ""] # [doc = " This trait is required by the [`Parser::try_map`] combinator."] pub trait FromExternalError < I , E > { # [doc = " Like [`ParserError::from_input`] but also include an external error."] fn from_external_error (input : & I , e : E) -> Self ; }
};
}
