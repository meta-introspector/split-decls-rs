// Generated macro for impl_1348 (impl)
macro_rules! Depcrate_spannedimpl_1348 {
() => {
// Module: crate::spanned
// Provides: {"impl_1348"}
// Dependencies: {}
impl Spanned for MacroArg { fn span (& self) -> Span { match * self { MacroArg :: Expr (ref expr) => expr . span () , MacroArg :: Ty (ref ty) => ty . span () , MacroArg :: Pat (ref pat) => pat . span () , MacroArg :: Item (ref item) => item . span () , MacroArg :: Keyword (_ , span) => span , } } }
};
}
