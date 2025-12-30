// Generated macro for rewrite_macro_name (function)
macro_rules! Depcrate_macrosrewrite_macro_name {
() => {
// Module: crate::macros
// Provides: {"rewrite_macro_name"}
// Dependencies: {}
# [doc = " Rewrite macro name without using pretty-printer if possible."] fn rewrite_macro_name (context : & RewriteContext < '_ > , path : & ast :: Path) -> String { if path . segments . len () == 1 { format ! ("{}!" , rewrite_ident (context , path . segments [0] . ident)) } else { format ! ("{}!" , pprust :: path_to_string (path)) } }
};
}
