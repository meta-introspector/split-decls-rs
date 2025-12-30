// Generated macro for parse_not (function)
macro_rules! Depcrate_exprparse_not {
() => {
// Module: crate::expr
// Provides: {"parse_not"}
// Dependencies: {}
fn parse_not (introducer : & Ident , iter : Iter) -> Result < Expr > { let paren = token :: parse_paren (introducer , iter) ? ; let ref mut inner = iter :: new (paren . stream ()) ; let expr = self :: parse (inner) ? ; token :: parse_optional_punct (inner , ',') ; token :: parse_end (inner) ? ; Ok (Expr :: Not (Box :: new (expr))) }
};
}
