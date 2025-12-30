// Generated macro for impl_837 (impl)
macro_rules! Depcrate_macrosimpl_837 {
() => {
// Module: crate::macros
// Provides: {"impl_837"}
// Dependencies: {}
impl Rewrite for MacroArg { fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { self . rewrite_result (context , shape) . ok () } fn rewrite_result (& self , context : & RewriteContext < '_ > , shape : Shape) -> RewriteResult { match * self { MacroArg :: Expr (ref expr) => expr . rewrite_result (context , shape) , MacroArg :: Ty (ref ty) => ty . rewrite_result (context , shape) , MacroArg :: Pat (ref pat) => pat . rewrite_result (context , shape) , MacroArg :: Item (ref item) => item . rewrite_result (context , shape) , MacroArg :: Keyword (ident , _) => Ok (ident . name . to_string ()) , } } }
};
}
