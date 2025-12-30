// Generated macro for impl_21 (impl)
macro_rules! Depcrate_spannedimpl_21 {
() => {
// Module: crate::spanned
// Provides: {"impl_21"}
// Dependencies: {}
impl < T : PartialOrd > PartialOrd for Spanned < T > { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { self . value . partial_cmp (& other . value) } }
};
}
