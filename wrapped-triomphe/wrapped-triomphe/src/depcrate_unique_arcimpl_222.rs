// Generated macro for impl_222 (impl)
macro_rules! Depcrate_unique_arcimpl_222 {
() => {
// Module: crate::unique_arc
// Provides: {"impl_222"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < T : Serialize > Serialize for UniqueArc < T > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : :: serde :: ser :: Serializer , { (* * self) . serialize (serializer) } }
};
}
