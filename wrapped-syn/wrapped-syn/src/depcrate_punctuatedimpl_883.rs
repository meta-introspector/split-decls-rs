// Generated macro for impl_883 (impl)
macro_rules! Depcrate_punctuatedimpl_883 {
() => {
// Module: crate::punctuated
// Provides: {"impl_883"}
// Dependencies: {}
impl < T , P > FromIterator < Pair < T , P > > for Punctuated < T , P > { fn from_iter < I : IntoIterator < Item = Pair < T , P > > > (i : I) -> Self { let mut ret = Punctuated :: new () ; do_extend (& mut ret , i . into_iter ()) ; ret } }
};
}
