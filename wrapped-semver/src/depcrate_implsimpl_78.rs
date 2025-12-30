// Generated macro for impl_78 (impl)
macro_rules! Depcrate_implsimpl_78 {
() => {
// Module: crate::impls
// Provides: {"impl_78"}
// Dependencies: {}
impl FromIterator < Comparator > for VersionReq { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = Comparator > , { let comparators = Vec :: from_iter (iter) ; VersionReq { comparators } } }
};
}
