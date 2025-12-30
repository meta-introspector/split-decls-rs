// Generated macro for impl_349 (impl)
macro_rules! Depcrate_internedimpl_349 {
() => {
// Module: crate::interned
// Provides: {"impl_349"}
// Dependencies: {}
impl < A : Hash + Eq + PartialEq < T > + Clone + Lookup < T > , T > Lookup < Vec < T > > for & [A] { fn into_owned (self) -> Vec < T > { self . iter () . map (| a | Lookup :: into_owned (a . clone ())) . collect () } }
};
}
