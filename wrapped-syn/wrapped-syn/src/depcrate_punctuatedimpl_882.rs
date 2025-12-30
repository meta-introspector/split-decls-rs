// Generated macro for impl_882 (impl)
macro_rules! Depcrate_punctuatedimpl_882 {
() => {
// Module: crate::punctuated
// Provides: {"impl_882"}
// Dependencies: {}
impl < T , P > Extend < T > for Punctuated < T , P > where P : Default , { fn extend < I : IntoIterator < Item = T > > (& mut self , i : I) { for value in i { self . push (value) ; } } }
};
}
