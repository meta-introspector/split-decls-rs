// Generated macro for extract_pats_and_cond (function)
macro_rules! Depcrate_exprextract_pats_and_cond {
() => {
// Module: crate::expr
// Provides: {"extract_pats_and_cond"}
// Dependencies: {}
fn extract_pats_and_cond (expr : & ast :: Expr) -> (Option < & ast :: Pat > , & ast :: Expr) { match expr . kind { ast :: ExprKind :: Let (ref pat , ref cond , _ , _) => (Some (pat) , cond) , _ => (None , expr) , } }
};
}
