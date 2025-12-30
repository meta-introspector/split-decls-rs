// Generated macro for parse_end (function)
macro_rules! Depcrate_tokenparse_end {
() => {
// Module: crate::token
// Provides: {"parse_end"}
// Dependencies: {}
pub fn parse_end (iter : Iter) -> Result < () > { match iter . next () { None => Ok (()) , Some (unexpected) => Err (Error :: new (unexpected . span () , "unexpected token")) , } }
};
}
