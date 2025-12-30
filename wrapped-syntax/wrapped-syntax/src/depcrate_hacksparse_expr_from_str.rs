// Generated macro for parse_expr_from_str (function)
macro_rules! Depcrate_hacksparse_expr_from_str {
() => {
// Module: crate::hacks
// Provides: {"parse_expr_from_str"}
// Dependencies: {}
pub fn parse_expr_from_str (s : & str , edition : Edition) -> Option < ast :: Expr > { let s = s . trim () ; let file = ast :: SourceFile :: parse (& format ! ("const _: () = ({s}\n);") , edition ,) ; let expr = file . syntax_node () . descendants () . find_map (ast :: ParenExpr :: cast) ? ; expr . expr () }
};
}
