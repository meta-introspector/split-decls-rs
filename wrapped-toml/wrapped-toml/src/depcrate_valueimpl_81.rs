// Generated macro for impl_81 (impl)
macro_rules! Depcrate_valueimpl_81 {
() => {
// Module: crate::value
// Provides: {"impl_81"}
// Dependencies: {}
impl < S : Into < String > , V : Into < Self > > From < BTreeMap < S , V > > for Value { fn from (val : BTreeMap < S , V >) -> Self { let table = val . into_iter () . map (| (s , v) | (s . into () , v . into ())) . collect () ; Self :: Table (table) } }
};
}
