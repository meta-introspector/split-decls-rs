// Generated macro for get_span_without_attrs (function)
macro_rules! Depcrate_attrget_span_without_attrs {
() => {
// Module: crate::attr
// Provides: {"get_span_without_attrs"}
// Dependencies: {}
pub (crate) fn get_span_without_attrs (stmt : & ast :: Stmt) -> Span { match stmt . kind { ast :: StmtKind :: Let (ref local) => local . span , ast :: StmtKind :: Item (ref item) => item . span , ast :: StmtKind :: Expr (ref expr) | ast :: StmtKind :: Semi (ref expr) => expr . span , ast :: StmtKind :: MacCall (ref mac_stmt) => mac_stmt . mac . span () , ast :: StmtKind :: Empty => stmt . span , } }
};
}
