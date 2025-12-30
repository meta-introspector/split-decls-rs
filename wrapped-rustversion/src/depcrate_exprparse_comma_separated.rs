// Generated macro for parse_comma_separated (function)
macro_rules! Depcrate_exprparse_comma_separated {
() => {
// Module: crate::expr
// Provides: {"parse_comma_separated"}
// Dependencies: {}
fn parse_comma_separated (iter : Iter) -> Result < Vec < Expr > > { let mut exprs = Vec :: new () ; while iter . peek () . is_some () { let expr = self :: parse (iter) ? ; exprs . push (expr) ; if iter . peek () . is_none () { break ; } token :: parse_punct (iter , ',') ? ; } Ok (exprs) }
};
}
