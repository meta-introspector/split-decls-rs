// Generated macro for impl_138 (impl)
macro_rules! Depcrate_durabilityimpl_138 {
() => {
// Module: crate::durability
// Provides: {"impl_138"}
// Dependencies: {}
# [cfg (feature = "persistence")] impl serde :: Serialize for Durability { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { serde :: Serialize :: serialize (& (self . 0 as u8) , serializer) } }
};
}
