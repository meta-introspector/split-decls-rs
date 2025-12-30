// Generated macro for impl_351 (impl)
macro_rules! Depcrate_internedimpl_351 {
() => {
// Module: crate::interned
// Provides: {"impl_351"}
// Dependencies: {}
impl < const N : usize , A : Hash + Eq + PartialEq < T > + Clone + Lookup < T > , T > Lookup < Vec < T > > for [A ; N] { fn into_owned (self) -> Vec < T > { self . into_iter () . map (| a | Lookup :: into_owned (a . clone ())) . collect () } }
};
}
