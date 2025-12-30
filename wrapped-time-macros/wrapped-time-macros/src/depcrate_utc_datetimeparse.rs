// Generated macro for parse (function)
macro_rules! Depcrate_utc_datetimeparse {
() => {
// Module: crate::utc_datetime
// Provides: {"parse"}
// Dependencies: {}
pub (crate) fn parse (chars : & mut Peekable < token_stream :: IntoIter >) -> Result < UtcDateTime , Error > { let date = date :: parse (chars) ? ; let time = time :: parse (chars) ? ; if let Some (token) = chars . peek () { return Err (Error :: UnexpectedToken { tree : token . clone () , }) ; } Ok (UtcDateTime { date , time }) }
};
}
