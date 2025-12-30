// Generated macro for parse_any (function)
macro_rules! Depcrate_exprparse_any {
() => {
// Module: crate::expr
// Provides: {"parse_any"}
// Dependencies: {}
fn parse_any (introducer : & Ident , iter : Iter) -> Result < Expr > { let paren = token :: parse_paren (introducer , iter) ? ; let ref mut inner = iter :: new (paren . stream ()) ; let exprs = parse_comma_separated (inner) ? ; Ok (Expr :: Any (exprs . into_iter () . collect ())) }
};
}
