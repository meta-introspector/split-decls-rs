// Generated macro for impl_26 (impl)
macro_rules! Depcrate_delimitedimpl_26 {
() => {
// Module: crate::delimited
// Provides: {"impl_26"}
// Dependencies: {}
impl < T , D > FromIterator < Element < T , D > > for Delimited < T , D > { fn from_iter < I : IntoIterator < Item = Element < T , D > > > (i : I) -> Self { let mut ret = Delimited :: new () ; ret . extend (i) ; ret } }
};
}
