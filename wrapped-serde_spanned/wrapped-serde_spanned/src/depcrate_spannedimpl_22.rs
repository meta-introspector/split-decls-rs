// Generated macro for impl_22 (impl)
macro_rules! Depcrate_spannedimpl_22 {
() => {
// Module: crate::spanned
// Provides: {"impl_22"}
// Dependencies: {}
impl < T : Ord > Ord for Spanned < T > { fn cmp (& self , other : & Self) -> Ordering { self . value . cmp (& other . value) } }
};
}
