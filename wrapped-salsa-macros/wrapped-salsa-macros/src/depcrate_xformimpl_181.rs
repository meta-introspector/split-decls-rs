// Generated macro for impl_181 (impl)
macro_rules! Depcrate_xformimpl_181 {
() => {
// Module: crate::xform
// Provides: {"impl_181"}
// Dependencies: {}
impl syn :: visit_mut :: VisitMut for ChangeLt < '_ > { fn visit_lifetime_mut (& mut self , i : & mut syn :: Lifetime) { if self . from . map (| f | i . ident == f) . unwrap_or (true) { i . ident = syn :: Ident :: new (& self . to , i . ident . span ()) ; } } }
};
}
