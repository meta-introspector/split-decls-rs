// Generated macro for impl_50 (impl)
macro_rules! Depcrate_mappableimpl_50 {
() => {
// Module: crate::mappable
// Provides: {"impl_50"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < K : Valuable , V : Valuable > Mappable for alloc :: collections :: BTreeMap < K , V > { fn size_hint (& self) -> (usize , Option < usize >) { self . iter () . size_hint () } }
};
}
