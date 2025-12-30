// Generated macro for Result (type)
macro_rules! Depcrate_errorResult {
() => {
// Module: crate::error
// Provides: {"Result"}
// Dependencies: {}
# [doc = " By default, the error type (`E`) is [`ContextError`]."] # [doc = ""] # [doc = " When integrating into the result of the application, see"] # [doc = " - [`Parser::parse`]"] # [doc = " - [`ParserError::into_inner`]"] pub type Result < O , E = ContextError > = core :: result :: Result < O , E > ;
};
}
