// Generated macro for impl_399 (impl)
macro_rules! Depcrate_revisionimpl_399 {
() => {
// Module: crate::revision
// Provides: {"impl_399"}
// Dependencies: {}
# [cfg (feature = "persistence")] impl serde :: Serialize for AtomicRevision { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { serde :: Serialize :: serialize (& self . data . load (Ordering :: Relaxed) , serializer) } }
};
}
