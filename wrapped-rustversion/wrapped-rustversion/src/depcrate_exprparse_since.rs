// Generated macro for parse_since (function)
macro_rules! Depcrate_exprparse_since {
() => {
// Module: crate::expr
// Provides: {"parse_since"}
// Dependencies: {}
fn parse_since (introducer : & Ident , iter : Iter) -> Result < Expr > { let paren = token :: parse_paren (introducer , iter) ? ; let ref mut inner = iter :: new (paren . stream ()) ; let bound = bound :: parse (paren , inner) ? ; token :: parse_optional_punct (inner , ',') ; token :: parse_end (inner) ? ; Ok (Expr :: Since (bound)) }
};
}
