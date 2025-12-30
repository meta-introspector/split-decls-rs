// Generated macro for impl_881 (impl)
macro_rules! Depcrate_punctuatedimpl_881 {
() => {
// Module: crate::punctuated
// Provides: {"impl_881"}
// Dependencies: {}
impl < T , P > FromIterator < T > for Punctuated < T , P > where P : Default , { fn from_iter < I : IntoIterator < Item = T > > (i : I) -> Self { let mut ret = Punctuated :: new () ; ret . extend (i) ; ret } }
};
}
