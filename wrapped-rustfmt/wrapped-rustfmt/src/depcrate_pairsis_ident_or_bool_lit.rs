// Generated macro for is_ident_or_bool_lit (function)
macro_rules! Depcrate_pairsis_ident_or_bool_lit {
() => {
// Module: crate::pairs
// Provides: {"is_ident_or_bool_lit"}
// Dependencies: {}
fn is_ident_or_bool_lit (expr : & ast :: Expr) -> bool { match & expr . kind { ast :: ExprKind :: Path (None , path) if path . segments . len () == 1 => true , ast :: ExprKind :: Lit (token :: Lit { kind : token :: LitKind :: Bool , .. }) => true , ast :: ExprKind :: Unary (_ , expr) | ast :: ExprKind :: AddrOf (_ , _ , expr) | ast :: ExprKind :: Paren (expr) | ast :: ExprKind :: Try (expr) => is_ident_or_bool_lit (expr) , _ => false , } }
};
}
