// Generated macro for impl_82 (impl)
macro_rules! Depcrate_valueimpl_82 {
() => {
// Module: crate::value
// Provides: {"impl_82"}
// Dependencies: {}
# [cfg (feature = "std")] impl < S : Into < String > + Hash + Eq , V : Into < Self > > From < HashMap < S , V > > for Value { fn from (val : HashMap < S , V >) -> Self { let table = val . into_iter () . map (| (s , v) | (s . into () , v . into ())) . collect () ; Self :: Table (table) } }
};
}
