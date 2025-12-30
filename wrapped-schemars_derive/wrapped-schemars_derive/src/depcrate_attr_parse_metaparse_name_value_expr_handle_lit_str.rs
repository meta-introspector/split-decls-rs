// Generated macro for parse_name_value_expr_handle_lit_str (function)
macro_rules! Depcrate_attr_parse_metaparse_name_value_expr_handle_lit_str {
() => {
// Module: crate::attr::parse_meta
// Provides: {"parse_name_value_expr_handle_lit_str"}
// Dependencies: {}
pub fn parse_name_value_expr_handle_lit_str (meta : CustomMeta , cx : & AttrCtxt) -> Result < Expr , () > { let expr = parse_name_value_expr (meta , cx) ? ; if let Expr :: Lit (ExprLit { lit : Lit :: Str (lit_str) , .. }) = & expr { parse_lit_str (lit_str , cx) } else { Ok (expr) } }
};
}
