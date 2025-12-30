// Generated macro for impl_101 (impl)
macro_rules! Depcrate_cycleimpl_101 {
() => {
// Module: crate::cycle
// Provides: {"impl_101"}
// Dependencies: {}
impl < 'a > std :: iter :: IntoIterator for & 'a CycleHeads { type Item = & 'a CycleHead ; type IntoIter = CycleHeadsIterator < 'a > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
