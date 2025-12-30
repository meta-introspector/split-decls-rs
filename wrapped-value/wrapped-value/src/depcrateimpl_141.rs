// Generated macro for impl_141 (impl)
macro_rules! Depcrateimpl_141 {
() => {
// Module: crate
// Provides: {"impl_141"}
// Dependencies: {}
impl < T : Into < ConstValue > > From < Vec < T > > for ConstValue { fn from (f : Vec < T >) -> Self { ConstValue :: List (f . into_iter () . map (Into :: into) . collect ()) } }
};
}
