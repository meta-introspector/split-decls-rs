// Generated macro for parse_error (function)
macro_rules! Depcrateparse_error {
() => {
// Module: crate
// Provides: {"parse_error"}
// Dependencies: {}
# [doc = " An error with a default error message."] # [doc = ""] # [doc = " NOTE: We should provide better error messages in the future."] pub fn parse_error < O > () -> PResult < 'static , O > { Err (ParseError (None)) }
};
}
