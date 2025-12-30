// Generated macro for impl_215 (impl)
macro_rules! Depcrate_idimpl_215 {
() => {
// Module: crate::id
// Provides: {"impl_215"}
// Dependencies: {}
# [cfg (feature = "persistence")] impl serde :: Serialize for Id { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { serde :: Serialize :: serialize (& self . as_bits () , serializer) } }
};
}
