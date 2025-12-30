// Generated macro for ModalResult (type)
macro_rules! Depcrate_errorModalResult {
() => {
// Module: crate::error
// Provides: {"ModalResult"}
// Dependencies: {}
# [doc = " [Modal error reporting][ErrMode] for [`Parser::parse_next`]"] # [doc = ""] # [doc = " - `Ok(O)` is the parsed value"] # [doc = " - [`Err(ErrMode<E>)`][ErrMode] is the error along with how to respond to it"] # [doc = ""] # [doc = " By default, the error type (`E`) is [`ContextError`]."] # [doc = ""] # [doc = " When integrating into the result of the application, see"] # [doc = " - [`Parser::parse`]"] # [doc = " - [`ParserError::into_inner`]"] pub type ModalResult < O , E = ContextError > = Result < O , ErrMode < E > > ;
};
}
