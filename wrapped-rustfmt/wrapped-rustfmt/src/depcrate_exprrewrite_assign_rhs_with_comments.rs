// Generated macro for rewrite_assign_rhs_with_comments (function)
macro_rules! Depcrate_exprrewrite_assign_rhs_with_comments {
() => {
// Module: crate::expr
// Provides: {"rewrite_assign_rhs_with_comments"}
// Dependencies: {}
pub (crate) fn rewrite_assign_rhs_with_comments < S : Into < String > , R : Rewrite + Spanned > (context : & RewriteContext < '_ > , lhs : S , ex : & R , shape : Shape , rhs_kind : & RhsAssignKind < '_ > , rhs_tactics : RhsTactics , between_span : Span , allow_extend : bool ,) -> RewriteResult { let lhs = lhs . into () ; let contains_comment = contains_comment (context . snippet (between_span)) ; let shape = if contains_comment { shape . block_left (context . config . tab_spaces () , between_span . with_hi (ex . span () . hi ()) ,) ? } else { shape } ; let rhs = rewrite_assign_rhs_expr (context , & lhs , ex , shape , rhs_kind , rhs_tactics) ? ; if contains_comment { let rhs = rhs . trim_start () ; combine_strs_with_missing_comments (context , & lhs , rhs , between_span , shape , allow_extend) } else { Ok (lhs + & rhs) } }
};
}
