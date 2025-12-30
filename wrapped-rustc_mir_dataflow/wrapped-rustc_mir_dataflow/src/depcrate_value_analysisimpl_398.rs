// Generated macro for impl_398 (impl)
macro_rules! Depcrate_value_analysisimpl_398 {
() => {
// Module: crate::value_analysis
// Provides: {"impl_398"}
// Dependencies: {}
impl < V : HasBottom > StateData < V > { fn new () -> StateData < V > { StateData { bottom : V :: BOTTOM , map : FxHashMap :: default () } } fn get (& self , idx : ValueIndex) -> & V { self . map . get (& idx) . unwrap_or (& self . bottom) } fn insert (& mut self , idx : ValueIndex , elem : V) { if elem . is_bottom () { self . map . remove (& idx) ; } else { self . map . insert (idx , elem) ; } } }
};
}
