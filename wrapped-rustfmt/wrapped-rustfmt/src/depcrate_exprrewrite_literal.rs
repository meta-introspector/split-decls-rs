// Generated macro for rewrite_literal (function)
macro_rules! Depcrate_exprrewrite_literal {
() => {
// Module: crate::expr
// Provides: {"rewrite_literal"}
// Dependencies: {}
pub (crate) fn rewrite_literal (context : & RewriteContext < '_ > , token_lit : token :: Lit , span : Span , shape : Shape ,) -> RewriteResult { match token_lit . kind { token :: LitKind :: Str => rewrite_string_lit (context , span , shape) , token :: LitKind :: Integer => rewrite_int_lit (context , token_lit , span , shape) , token :: LitKind :: Float => rewrite_float_lit (context , token_lit , span , shape) , _ => wrap_str (context . snippet (span) . to_owned () , context . config . max_width () , shape ,) . max_width_error (shape . width , span) , } }
};
}
