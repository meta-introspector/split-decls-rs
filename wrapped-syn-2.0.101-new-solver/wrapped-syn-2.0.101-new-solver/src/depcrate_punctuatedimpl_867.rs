// Generated macro for impl_867 (impl)
macro_rules! Depcrate_punctuatedimpl_867 {
() => {
// Module: crate::punctuated
// Provides: {"impl_867"}
// Dependencies: {}
impl < T , P > Extend < Pair < T , P > > for Punctuated < T , P > where P : Default , { fn extend < I : IntoIterator < Item = Pair < T , P > > > (& mut self , i : I) { if ! self . empty_or_trailing () { self . push_punct (P :: default ()) ; } do_extend (self , i . into_iter ()) ; } }
};
}
