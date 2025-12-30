// Generated macro for rewrite_expr_addrof (function)
macro_rules! Depcrate_exprrewrite_expr_addrof {
() => {
// Module: crate::expr
// Provides: {"rewrite_expr_addrof"}
// Dependencies: {}
fn rewrite_expr_addrof (context : & RewriteContext < '_ > , borrow_kind : ast :: BorrowKind , mutability : ast :: Mutability , expr : & ast :: Expr , shape : Shape ,) -> RewriteResult { let operator_str = match (mutability , borrow_kind) { (ast :: Mutability :: Not , ast :: BorrowKind :: Ref) => "&" , (ast :: Mutability :: Not , ast :: BorrowKind :: Raw) => "&raw const " , (ast :: Mutability :: Mut , ast :: BorrowKind :: Ref) => "&mut " , (ast :: Mutability :: Mut , ast :: BorrowKind :: Raw) => "&raw mut " , } ; rewrite_unary_prefix (context , operator_str , expr , shape) }
};
}
