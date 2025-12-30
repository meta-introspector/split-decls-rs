// Generated macro for rewrite_closure_with_block (function)
macro_rules! Depcrate_closuresrewrite_closure_with_block {
() => {
// Module: crate::closures
// Provides: {"rewrite_closure_with_block"}
// Dependencies: {}
fn rewrite_closure_with_block (body : & ast :: Expr , prefix : & str , context : & RewriteContext < '_ > , shape : Shape ,) -> RewriteResult { let left_most = left_most_sub_expr (body) ; let veto_block = veto_block (body) && ! expr_requires_semi_to_be_stmt (left_most) ; if veto_block { return Err (RewriteError :: Unknown) ; } let block = ast :: Block { stmts : thin_vec ! [ast :: Stmt { id : ast :: NodeId :: root () , kind : ast :: StmtKind :: Expr (ptr :: P (body . clone ())) , span : body . span , }] , id : ast :: NodeId :: root () , rules : ast :: BlockCheckMode :: Default , tokens : None , span : body . attrs . first () . map (| attr | attr . span . to (body . span)) . unwrap_or (body . span) , } ; let block = crate :: expr :: rewrite_block_with_visitor (context , "" , & block , Some (& body . attrs) , None , shape , false ,) ? ; Ok (format ! ("{prefix} {block}")) }
};
}
