// Generated macro for rewrite_ident (function)
macro_rules! Depcrate_utilsrewrite_ident {
() => {
// Module: crate::utils
// Provides: {"rewrite_ident"}
// Dependencies: {}
pub (crate) fn rewrite_ident < 'a > (context : & 'a RewriteContext < '_ > , ident : symbol :: Ident) -> & 'a str { context . snippet (ident . span) }
};
}
