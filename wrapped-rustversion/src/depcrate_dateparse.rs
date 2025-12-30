// Generated macro for parse (function)
macro_rules! Depcrate_dateparse {
() => {
// Module: crate::date
// Provides: {"parse"}
// Dependencies: {}
pub fn parse (paren : Group , iter : Iter) -> Result < Date > { try_parse (iter) . map_err (| () | { let msg = format ! ("expected nightly date, like {}" , time :: today ()) ; Error :: group (paren , msg) }) }
};
}
