// Generated macro for UnindexMap (type)
macro_rules! Depcrate_unhashUnindexMap {
() => {
// Module: crate::unhash
// Provides: {"UnindexMap"}
// Dependencies: {}
pub type UnindexMap < K , V > = indexmap :: IndexMap < K , V , BuildHasherDefault < Unhasher > > ;
};
}
