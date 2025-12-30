// Generated macro for impl_654 (impl)
macro_rules! Depcrate_importsimpl_654 {
() => {
// Module: crate::imports
// Provides: {"impl_654"}
// Dependencies: {}
impl Spanned for UseTree { fn span (& self) -> Span { let lo = if let Some (ref attrs) = self . attrs { attrs . iter () . next () . map_or (self . span . lo () , | a | a . span . lo ()) } else { self . span . lo () } ; mk_sp (lo , self . span . hi ()) } }
};
}
