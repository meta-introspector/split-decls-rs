// Generated macro for impl_889 (impl)
macro_rules! Depcrate_matchesimpl_889 {
() => {
// Module: crate::matches
// Provides: {"impl_889"}
// Dependencies: {}
impl < 'a > Spanned for ArmWrapper < 'a > { fn span (& self) -> Span { if let Some (lo) = self . beginning_vert { let lo = std :: cmp :: min (lo , self . arm . span () . lo ()) ; mk_sp (lo , self . arm . span () . hi ()) } else { self . arm . span () } } }
};
}
