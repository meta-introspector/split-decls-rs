// Generated macro for impl_139 (impl)
macro_rules! Depcrateimpl_139 {
() => {
// Module: crate
// Provides: {"impl_139"}
// Dependencies: {}
impl < T : Into < ConstValue > > FromIterator < T > for ConstValue { fn from_iter < I : IntoIterator < Item = T > > (iter : I) -> Self { ConstValue :: List (iter . into_iter () . map (Into :: into) . collect ()) } }
};
}
