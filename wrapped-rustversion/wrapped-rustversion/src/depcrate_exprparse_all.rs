// Generated macro for parse_all (function)
macro_rules! Depcrate_exprparse_all {
() => {
// Module: crate::expr
// Provides: {"parse_all"}
// Dependencies: {}
fn parse_all (introducer : & Ident , iter : Iter) -> Result < Expr > { let paren = token :: parse_paren (introducer , iter) ? ; let ref mut inner = iter :: new (paren . stream ()) ; let exprs = parse_comma_separated (inner) ? ; Ok (Expr :: All (exprs . into_iter () . collect ())) }
};
}
