// Generated macro for impl_16 (impl)
macro_rules! Depcrate_arrayimpl_16 {
() => {
// Module: crate::array
// Provides: {"impl_16"}
// Dependencies: {}
impl < V : Into < Value > > FromIterator < V > for Array { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = V > , { let v = iter . into_iter () . map (| a | Item :: Value (a . into ())) ; Self { values : v . collect () , .. Default :: default () } } }
};
}
