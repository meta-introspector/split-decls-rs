// Generated macro for Expr (enum)
macro_rules! Depcrate_exprExpr {
() => {
// Module: crate::expr
// Provides: {"Expr"}
// Dependencies: {}
pub enum Expr { Stable , Beta , Nightly , Date (Date) , Since (Bound) , Before (Bound) , Release (Release) , Not (Box < Expr >) , Any (Vec < Expr >) , All (Vec < Expr >) , }
};
}
