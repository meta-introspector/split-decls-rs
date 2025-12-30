// Generated macro for rewrite_closure (function)
macro_rules! Depcrate_closuresrewrite_closure {
() => {
// Module: crate::closures
// Provides: {"rewrite_closure"}
// Dependencies: {}
pub (crate) fn rewrite_closure (binder : & ast :: ClosureBinder , constness : ast :: Const , capture : ast :: CaptureBy , coroutine_kind : & Option < ast :: CoroutineKind > , movability : ast :: Movability , fn_decl : & ast :: FnDecl , body : & ast :: Expr , span : Span , context : & RewriteContext < '_ > , shape : Shape ,) -> RewriteResult { debug ! ("rewrite_closure {:?}" , body) ; let (prefix , extra_offset) = rewrite_closure_fn_decl (binder , constness , capture , coroutine_kind , movability , fn_decl , body , span , context , shape ,) ? ; let body_shape = shape . offset_left (extra_offset , span) ? ; if let ast :: ExprKind :: Block (ref block , _) = body . kind { if block . stmts . is_empty () && ! block_contains_comment (context , block) { return body . rewrite_result (context , shape) . map (| s | format ! ("{} {}" , prefix , s)) ; } let result = match fn_decl . output { ast :: FnRetTy :: Default (_) if ! context . inside_macro () => { try_rewrite_without_block (body , & prefix , context , shape , body_shape) } _ => Err (RewriteError :: Unknown) , } ; result . or_else (| _ | { rewrite_closure_block (body , & prefix , context , body_shape) }) } else { rewrite_closure_expr (body , & prefix , context , body_shape) . or_else (| _ | { rewrite_closure_with_block (body , & prefix , context , body_shape) }) } }
};
}
