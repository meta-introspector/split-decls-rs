// Generated macro for parse (function)
macro_rules! Depcrate_parserparse {
() => {
// Module: crate::parser
// Provides: {"parse"}
// Dependencies: {}
pub (crate) fn parse (tokens : Vec < lexer :: Token >) -> Result < Grammar > { let mut p = Parser :: new (tokens) ; while ! p . is_eof () { node (& mut p) ? ; } p . finish () }
};
}
