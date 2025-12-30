// Generated macro for parse (function)
macro_rules! Depcrate_datetimeparse {
() => {
// Module: crate::datetime
// Provides: {"parse"}
// Dependencies: {}
pub (crate) fn parse (chars : & mut Peekable < token_stream :: IntoIter >) -> Result < DateTime , Error > { let date = date :: parse (chars) ? ; let time = time :: parse (chars) ? ; let offset = match offset :: parse (chars) { Ok (offset) => Some (offset) , Err (Error :: UnexpectedEndOfInput | Error :: MissingComponent { name : "sign" , .. }) => None , Err (err) => return Err (err) , } ; if let Some (token) = chars . peek () { return Err (Error :: UnexpectedToken { tree : token . clone () , }) ; } Ok (DateTime { date , time , offset }) }
};
}
