// Generated macro for impl_317 (impl)
macro_rules! Depcrate_tableimpl_317 {
() => {
// Module: crate::table
// Provides: {"impl_317"}
// Dependencies: {}
impl < K : Into < Key > , V : Into < Item > > FromIterator < (K , V) > for Table { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = (K , V) > , { let mut table = Self :: new () ; table . extend (iter) ; table } }
};
}
