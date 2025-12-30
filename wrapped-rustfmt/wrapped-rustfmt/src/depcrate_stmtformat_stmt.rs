// Generated macro for format_stmt (function)
macro_rules! Depcrate_stmtformat_stmt {
() => {
// Module: crate::stmt
// Provides: {"format_stmt"}
// Dependencies: {}
fn format_stmt (context : & RewriteContext < '_ > , shape : Shape , stmt : & ast :: Stmt , expr_type : ExprType , is_last_expr : bool ,) -> RewriteResult { skip_out_of_file_lines_range_err ! (context , stmt . span ()) ; let result = match stmt . kind { ast :: StmtKind :: Let (ref local) => local . rewrite_result (context , shape) , ast :: StmtKind :: Expr (ref ex) | ast :: StmtKind :: Semi (ref ex) => { let suffix = if semicolon_for_stmt (context , stmt , is_last_expr) { ";" } else { "" } ; let shape = shape . sub_width (suffix . len () , ex . span ()) ? ; format_expr (ex , expr_type , context , shape) . map (| s | s + suffix) } ast :: StmtKind :: MacCall (..) | ast :: StmtKind :: Item (..) | ast :: StmtKind :: Empty => { Err (RewriteError :: Unknown) } } ; result . map (| res | recover_comment_removed (res , stmt . span () , context)) }
};
}
