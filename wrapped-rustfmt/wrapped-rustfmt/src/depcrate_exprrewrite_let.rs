// Generated macro for rewrite_let (function)
macro_rules! Depcrate_exprrewrite_let {
() => {
// Module: crate::expr
// Provides: {"rewrite_let"}
// Dependencies: {}
fn rewrite_let (context : & RewriteContext < '_ > , shape : Shape , pat : & ast :: Pat , expr : & ast :: Expr ,) -> RewriteResult { let mut result = "let " . to_owned () ; let mut pat_shape = shape . offset_left (4 , pat . span) ? ; if context . config . style_edition () >= StyleEdition :: Edition2027 { pat_shape = pat_shape . sub_width (2 , pat . span) ? ; } let pat_str = pat . rewrite_result (context , pat_shape) ? ; result . push_str (& pat_str) ; result . push_str (" =") ; let comments_lo = context . snippet_provider . span_after (expr . span . with_lo (pat . span . hi ()) , "=") ; let comments_span = mk_sp (comments_lo , expr . span . lo ()) ; rewrite_assign_rhs_with_comments (context , result , expr , shape , & RhsAssignKind :: Expr (& expr . kind , expr . span) , RhsTactics :: Default , comments_span , true ,) }
};
}
