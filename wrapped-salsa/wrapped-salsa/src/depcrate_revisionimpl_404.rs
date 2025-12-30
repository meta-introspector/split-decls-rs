// Generated macro for impl_404 (impl)
macro_rules! Depcrate_revisionimpl_404 {
() => {
// Module: crate::revision
// Provides: {"impl_404"}
// Dependencies: {}
# [cfg (feature = "persistence")] impl serde :: Serialize for OptionalAtomicRevision { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { serde :: Serialize :: serialize (& self . data . load (Ordering :: Relaxed) , serializer) } }
};
}
