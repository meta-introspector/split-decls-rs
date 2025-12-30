// Generated macro for impl_27 (impl)
macro_rules! Depcrate_delimitedimpl_27 {
() => {
// Module: crate::delimited
// Provides: {"impl_27"}
// Dependencies: {}
impl < T , D > FromIterator < T > for Delimited < T , D > where D : Default , { fn from_iter < I : IntoIterator < Item = T > > (i : I) -> Self { let mut ret = Delimited :: new () ; ret . extend (i) ; ret } }
};
}
