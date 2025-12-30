// Generated macro for convert_try_mac (function)
macro_rules! Depcrate_macrosconvert_try_mac {
() => {
// Module: crate::macros
// Provides: {"convert_try_mac"}
// Dependencies: {}
# [doc = " Tries to convert a macro use into a short hand try expression. Returns `None`"] # [doc = " when the macro is not an instance of `try!` (or parsing the inner expression"] # [doc = " failed)."] pub (crate) fn convert_try_mac (mac : & ast :: MacCall , context : & RewriteContext < '_ > ,) -> Option < ast :: Expr > { let path = & pprust :: path_to_string (& mac . path) ; if path == "try" || path == "r#try" { let ts = mac . args . tokens . clone () ; Some (ast :: Expr { id : ast :: NodeId :: root () , kind : ast :: ExprKind :: Try (parse_expr (context , ts) ?) , span : mac . span () , attrs : ast :: AttrVec :: new () , tokens : None , }) } else { None } }
};
}
