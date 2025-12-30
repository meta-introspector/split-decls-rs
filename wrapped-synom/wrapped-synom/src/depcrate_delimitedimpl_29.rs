// Generated macro for impl_29 (impl)
macro_rules! Depcrate_delimitedimpl_29 {
() => {
// Module: crate::delimited
// Provides: {"impl_29"}
// Dependencies: {}
impl < T , D > Extend < T > for Delimited < T , D > where D : Default , { fn extend < I : IntoIterator < Item = T > > (& mut self , i : I) { for element in i { self . push_default (element) ; } } }
};
}
