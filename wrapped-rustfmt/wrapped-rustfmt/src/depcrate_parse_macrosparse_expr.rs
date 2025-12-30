// Generated macro for parse_expr (function)
macro_rules! Depcrate_parse_macrosparse_expr {
() => {
// Module: crate::parse::macros
// Provides: {"parse_expr"}
// Dependencies: {}
pub (crate) fn parse_expr (context : & RewriteContext < '_ > , tokens : TokenStream ,) -> Option < ptr :: P < ast :: Expr > > { let mut parser = build_parser (context , tokens) ; parser . parse_expr () . ok () }
};
}
