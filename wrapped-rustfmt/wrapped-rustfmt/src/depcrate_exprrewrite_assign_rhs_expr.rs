// Generated macro for rewrite_assign_rhs_expr (function)
macro_rules! Depcrate_exprrewrite_assign_rhs_expr {
() => {
// Module: crate::expr
// Provides: {"rewrite_assign_rhs_expr"}
// Dependencies: {}
pub (crate) fn rewrite_assign_rhs_expr < R : Rewrite > (context : & RewriteContext < '_ > , lhs : & str , ex : & R , shape : Shape , rhs_kind : & RhsAssignKind < '_ > , rhs_tactics : RhsTactics ,) -> RewriteResult { let last_line_width = last_line_width (lhs) . saturating_sub (if lhs . contains ('\n') { shape . indent . width () } else { 0 }) ; let orig_shape = shape . offset_left_opt (last_line_width + 1) . unwrap_or (Shape { width : 0 , offset : shape . offset + last_line_width + 1 , .. shape }) ; let has_rhs_comment = if let Some (offset) = lhs . find_last_uncommented ("=") { lhs . trim_end () . len () > offset + 1 } else { false } ; choose_rhs (context , ex , orig_shape , ex . rewrite_result (context , orig_shape) , rhs_kind , rhs_tactics , has_rhs_comment ,) }
};
}
