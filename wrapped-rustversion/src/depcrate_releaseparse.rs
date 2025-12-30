// Generated macro for parse (function)
macro_rules! Depcrate_releaseparse {
() => {
// Module: crate::release
// Provides: {"parse"}
// Dependencies: {}
pub fn parse (paren : Group , iter : Iter) -> Result < Release > { try_parse (iter) . map_err (| () | Error :: group (paren , "expected rustc release number, like 1.31")) }
};
}
