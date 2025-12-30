// Generated macro for impl_360 (impl)
macro_rules! Depcrate_valueimpl_360 {
() => {
// Module: crate::value
// Provides: {"impl_360"}
// Dependencies: {}
impl < V : Into < Self > > FromIterator < V > for Value { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = V > , { let array : Array = iter . into_iter () . collect () ; Self :: Array (array) } }
};
}
