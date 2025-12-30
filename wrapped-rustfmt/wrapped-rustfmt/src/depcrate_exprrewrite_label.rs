// Generated macro for rewrite_label (function)
macro_rules! Depcrate_exprrewrite_label {
() => {
// Module: crate::expr
// Provides: {"rewrite_label"}
// Dependencies: {}
fn rewrite_label (context : & RewriteContext < '_ > , opt_label : Option < ast :: Label >) -> Cow < 'static , str > { match opt_label { Some (label) => Cow :: from (format ! ("{}: " , context . snippet (label . ident . span))) , None => Cow :: from ("") , } }
};
}
