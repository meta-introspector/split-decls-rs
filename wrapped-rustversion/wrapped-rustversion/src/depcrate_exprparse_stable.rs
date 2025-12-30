// Generated macro for parse_stable (function)
macro_rules! Depcrate_exprparse_stable {
() => {
// Module: crate::expr
// Provides: {"parse_stable"}
// Dependencies: {}
fn parse_stable (iter : Iter) -> Result < Expr > { let paren = match token :: parse_optional_paren (iter) { Some (group) => group , None => return Ok (Expr :: Stable) , } ; let ref mut inner = iter :: new (paren . stream ()) ; let release = release :: parse (paren , inner) ? ; token :: parse_optional_punct (inner , ',') ; token :: parse_end (inner) ? ; Ok (Expr :: Release (release)) }
};
}
