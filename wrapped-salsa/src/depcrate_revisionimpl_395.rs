// Generated macro for impl_395 (impl)
macro_rules! Depcrate_revisionimpl_395 {
() => {
// Module: crate::revision
// Provides: {"impl_395"}
// Dependencies: {}
# [cfg (feature = "persistence")] impl serde :: Serialize for Revision { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { serde :: Serialize :: serialize (& self . as_usize () , serializer) } }
};
}
