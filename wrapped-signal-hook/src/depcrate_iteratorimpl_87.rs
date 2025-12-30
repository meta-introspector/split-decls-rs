// Generated macro for impl_87 (impl)
macro_rules! Depcrate_iteratorimpl_87 {
() => {
// Module: crate::iterator
// Provides: {"impl_87"}
// Dependencies: {}
impl < 'a , E : Exfiltrator > IntoIterator for & 'a mut SignalsInfo < E > { type Item = E :: Output ; type IntoIter = Forever < 'a , E > ; fn into_iter (self) -> Self :: IntoIter { self . forever () } }
};
}
