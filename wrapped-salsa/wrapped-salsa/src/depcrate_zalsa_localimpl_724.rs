// Generated macro for impl_724 (impl)
macro_rules! Depcrate_zalsa_localimpl_724 {
() => {
// Module: crate::zalsa_local
// Provides: {"impl_724"}
// Dependencies: {}
# [cfg (feature = "persistence")] impl serde :: Serialize for QueryOrigin { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { self . as_ref () . serialize (serializer) } }
};
}
