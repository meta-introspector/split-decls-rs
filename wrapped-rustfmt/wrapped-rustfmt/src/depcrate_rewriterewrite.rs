// Generated macro for Rewrite (trait)
macro_rules! Depcrate_rewriteRewrite {
() => {
// Module: crate::rewrite
// Provides: {"Rewrite"}
// Dependencies: {}
pub (crate) trait Rewrite { # [doc = " Rewrite self into shape."] fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > ; fn rewrite_result (& self , context : & RewriteContext < '_ > , shape : Shape) -> RewriteResult { self . rewrite (context , shape) . unknown_error () } }
};
}
