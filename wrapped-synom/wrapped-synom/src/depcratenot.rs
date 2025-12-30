// Generated macro for not (macro)
macro_rules! Depcratenot {
() => {
// Module: crate
// Provides: {"not"}
// Dependencies: {}
# [doc = " Parses successfully if the given parser fails to parse. Does not consume any"] # [doc = " of the input."] # [doc = ""] # [doc = " - **Syntax:** `not!(THING)`"] # [doc = " - **Output:** `()`"] # [macro_export] macro_rules ! not { ($ i : expr , $ submac : ident ! ($ ($ args : tt) *)) => { match $ submac ! ($ i , $ ($ args) *) { :: std :: result :: Result :: Ok (_) => $ crate :: parse_error () , :: std :: result :: Result :: Err (_) => :: std :: result :: Result :: Ok (($ i , ())) , } } ; }
};
}
