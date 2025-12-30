// Generated macro for impl_361 (impl)
macro_rules! Depcrate_valueimpl_361 {
() => {
// Module: crate::value
// Provides: {"impl_361"}
// Dependencies: {}
impl < K : Into < Key > , V : Into < Self > > FromIterator < (K , V) > for Value { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = (K , V) > , { let table : InlineTable = iter . into_iter () . collect () ; Self :: InlineTable (table) } }
};
}
